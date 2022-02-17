// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// Parser for `FontFeatureValuesRule`. Parses all blocks
/// <feature-type> {
///   <feature-value-declaration-list>
/// }
/// <feature-type> = @stylistic | @historical-forms | @styleset |
/// @character-variant | @swash | @ornaments | @annotation
pub(crate) struct FontFeatureValuesAtRuleParser<'a>
{
	pub(crate) context: &'a ParserContext,
	pub(crate) rule: &'a mut FontFeatureValuesAtRule,
}

/// Default methods reject all qualified rules.
impl<'a, 'i> QualifiedRuleParser<'i> for FontFeatureValuesAtRuleParser<'a>
{
	type Prelude = ();
	
	type QualifiedRule = ();
	
	type Error = CustomParseError<'i>;
}

impl<'a, 'i> AtRuleParser<'i> for FontFeatureValuesAtRuleParser<'a>
{
	type Prelude = FontFeatureValuesBlockType;
	
	type AtRule = ();
	
	type Error = CustomParseError<'i>;
	
	fn parse_prelude<'t>(&mut self, name: CowRcStr<'i>, input: &mut Parser<'i, 't>) -> Result<Self::Prelude, ParseError<'i, Self::Error>>
	{
		use self::FontFeatureValuesBlockType::*;
		
		match_ignore_ascii_case!
		{
			&*name,
			
			"swash" => Ok(swash),
			
			"stylistic" => Ok(stylistic),
			
			"ornaments" => Ok(ornaments),
			
			"annotation" => Ok(annotation),
			
			"character-variant" => Ok(character_variant),
			
			"styleset" => Ok(styleset),
			
			_ => Err(input.new_error(BasicParseErrorKind::AtRuleBodyInvalid)),
		}
	}
	
	fn parse_block<'t>(&mut self, prelude: Self::Prelude, _: &ParserState, input: &mut Parser<'i, 't>) -> Result<Self::AtRule, ParseError<'i, Self::Error>>
	{
		use self::FontFeatureValuesBlockType::*;
		
		match prelude
		{
			swash => FontFeatureValuesDeclarationsParser::parseBlock(input, self.context, &mut self.rule.swash),
			stylistic => FontFeatureValuesDeclarationsParser::parseBlock(input, self.context, &mut self.rule.stylistic),
			ornaments => FontFeatureValuesDeclarationsParser::parseBlock(input, self.context, &mut self.rule.ornaments),
			annotation => FontFeatureValuesDeclarationsParser::parseBlock(input, self.context, &mut self.rule.annotation),
			character_variant => FontFeatureValuesDeclarationsParser::parseBlock(input, self.context, &mut self.rule.character_variant),
			styleset => FontFeatureValuesDeclarationsParser::parseBlock(input, self.context, &mut self.rule.styleset),
		}
	}
}
