// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct OurSelectorParser<'a>
{
	pub(crate) namespaces: Rc<Namespaces>,
	pub(crate) applyVendorPrefixToPseudoClasses: &'a HashMap<VendorPrefixablePseudoClassName, VendorPrefix>,
	pub(crate) applyVendorPrefixToPseudoElements: &'a HashMap<VendorPrefixablePseudoElementName, VendorPrefix>,
}

impl<'a, 'i> crate::selectors::parser::Parser<'i> for OurSelectorParser<'a>
{
	type Impl = OurSelectorImpl;
	
	type Error = CustomParseError<'i>;
	
	#[inline(always)]
	fn parse_non_ts_pseudo_class(&self, name: CowRcStr<'i>) -> Result<<Self::Impl as SelectorImpl>::NonTSPseudoClass, SelectorParseError<'i, Self::Error>>
	{
		NonTreeStructuralPseudoClass::parse_without_arguments(self.applyVendorPrefixToPseudoClasses, name)
	}
	
	#[inline(always)]
	fn parse_non_ts_functional_pseudo_class<'t>(&self, name: CowRcStr<'i>, arguments: &mut Parser<'i, 't>) -> Result<<Self::Impl as SelectorImpl>::NonTSPseudoClass, ParseError<'i, SelectorParseError<'i, Self::Error>>>
	{
		NonTreeStructuralPseudoClass::parse_with_arguments(self.applyVendorPrefixToPseudoClasses, name, arguments, self)
	}
	
	#[inline(always)]
	fn parse_pseudo_element(&self, name: CowRcStr<'i>) -> Result<<Self::Impl as SelectorImpl>::PseudoElement, SelectorParseError<'i, Self::Error>>
	{
		PseudoElement::parse_without_arguments(self.applyVendorPrefixToPseudoElements, name)
	}
	
	#[inline(always)]
	fn parse_functional_pseudo_element<'t>(&self, name: CowRcStr<'i>, input: &mut Parser<'i, 't>) -> Result<<Self::Impl as SelectorImpl>::PseudoElement, ParseError<'i, SelectorParseError<'i, Self::Error>>>
	{
		PseudoElement::parse_with_arguments(self.applyVendorPrefixToPseudoElements, name, input, self)
	}
	
	#[inline(always)]
	fn default_namespace(&self) -> Option<<Self::Impl as SelectorImpl>::NamespaceUrl>
	{
		self.namespaces.default_namespace()
	}
	
	#[inline(always)]
	fn namespace_for_prefix(&self, prefix: &<Self::Impl as SelectorImpl>::NamespacePrefix) -> Option<<Self::Impl as SelectorImpl>::NamespaceUrl>
	{
		self.namespaces.namespace_for_prefix(prefix)
	}
}

impl<'a> OurSelectorParser<'a>
{
	#[inline(always)]
	pub(crate) fn parse<'i, 't>(&self, input: &mut Parser<'i, 't>) -> Result<DeduplicatedSelectors, ParseError<'i, CustomParseError<'i>>>
	{
		self.parse_internal(input, |_| false)
	}
	
	#[inline(always)]
	pub(crate) fn parse_internal<'i, 't, F: Fn(&OurSelector) -> bool>(&self, input: &mut Parser<'i, 't>, isInvalidSelector: F) -> Result<DeduplicatedSelectors, ParseError<'i, CustomParseError<'i>>>
	{
		let selectors = self.parse_selectors(input).map_err(|error| error.into())?;
		
		if selectors.is_empty()
		{
			return Err(input.new_custom_error(CustomParseError::ThereAreNoSelectors));
		}
		
		let mut deduplicatedSelectors = OrderMap::with_capacity(selectors.len());
		for selector in selectors
		{
			let selectorCss = selector.to_css_string();
			if isInvalidSelector(&selector)
			{
				return Err(input.new_custom_error(CustomParseError::SelectorIsInvalidInContext(selectorCss)))
			}
			
			// Selector does not implement Eq or Hash... Grrr...
			deduplicatedSelectors.insert(selectorCss, selector);
		}
		
		let mut listOfSelectors = Vec::with_capacity(deduplicatedSelectors.len());
		for (_css, selector) in deduplicatedSelectors.drain(..)
		{
			listOfSelectors.push(selector)
		}
		
		Ok(DeduplicatedSelectors(listOfSelectors))
	}
	
	#[inline(always)]
	fn parse_selectors<'i, 't>(&self, input: &mut Parser<'i, 't>) -> Result<SmallVec<[OurSelector; 1]>, ParseError<'i, SelectorParseError<'i, CustomParseError<'i>>>>
	{
		let selectorList = SelectorList::parse(self, input)?;
		Ok(selectorList.0)
	}
}
