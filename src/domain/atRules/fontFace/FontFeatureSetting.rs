// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct FontFeatureSetting(String, u32);

impl FontFeatureSetting
{
	fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		let openTypeFeatureTag =
		{
			let openTypeFeatureTag = input.expect_string()?;
			if openTypeFeatureTag.len() != 4
			{
				let openTypeFeatureTag = openTypeFeatureTag.clone();
				return Err(input.new_custom_error(CustomParseError::FontFeatureSettingOpenTypeFeatureTagMustBeFourCharacters(openTypeFeatureTag)));
			}
			
			for character in openTypeFeatureTag.chars()
			{
				if character <= '\x20' || character > '\x7E'
				{
					let openTypeFeatureTag = openTypeFeatureTag.clone();
					return Err(input.new_custom_error(CustomParseError::FontFeatureSettingOpenTypeFeatureTagMustBePrintableAscii(openTypeFeatureTag)));
				}
			}
			
			openTypeFeatureTag.as_ref().into()
		};
		
		if let Ok(integer) = input.r#try(|input| input.expect_integer())
		{
			if integer < 0
			{
				Err(input.new_custom_error(CustomParseError::FontFeatureSettingIntegerMustBePositive(integer)))
			}
			else
			{
				Ok(FontFeatureSetting(openTypeFeatureTag, integer as u32))
			}
		}
		else
		{
			let ident = input.expect_ident()?;
			
			match_ignore_ascii_case!
			{
				&ident,
				
				"on" => Ok(FontFeatureSetting(openTypeFeatureTag, 1)),
				
				"off" => Ok(FontFeatureSetting(openTypeFeatureTag, 0)),
				
				_ => {
					let ident = ident.clone();
					Err(input.new_custom_error(CustomParseError::FontFeatureSettingIfNotAnIntegerMustBeOnOrOff(ident)))
				},
			}
		}
	}
}
