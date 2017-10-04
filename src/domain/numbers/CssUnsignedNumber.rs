// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A CSS float value similar to f32 but with a more restricted range
#[derive(Debug, Copy, Clone)]
pub struct CssUnsignedNumber(f32);

impl PartialEq for CssUnsignedNumber
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.to_f32().eq(&other.0)
	}
}

impl Eq for CssUnsignedNumber
{
}

impl PartialOrd for CssUnsignedNumber
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		self.to_f32().partial_cmp(&other.0)
	}
}

impl Ord for CssUnsignedNumber
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		self.partial_cmp(other).unwrap_or(Ordering::Equal)
	}
}

impl Hash for CssUnsignedNumber
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.to_bits().hash(state)
	}
}

impl ToCss for CssUnsignedNumber
{
	#[inline(always)]
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		self.to_f32().to_css(dest)
	}
}

impl Display for CssUnsignedNumber
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut Formatter) -> fmt::Result
	{
		<f32 as Display>::fmt(&self.to_f32(), fmt)
	}
}

impl LowerExp for CssUnsignedNumber
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<f32 as LowerExp>::fmt(&self.to_f32(), f)
	}
}

impl UpperExp for CssUnsignedNumber
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<f32 as UpperExp>::fmt(&self.to_f32(), f)
	}
}

impl Default for CssUnsignedNumber
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Zero
	}
}

impl Add<CssUnsignedNumber> for CssUnsignedNumber
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: CssUnsignedNumber) -> Self::Output
	{
		Self::clamp(self.to_f32() + rhs.0)
	}
}

impl AddAssign<CssUnsignedNumber> for CssUnsignedNumber
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: CssUnsignedNumber)
	{
		*self = self.add(rhs)
	}
}

impl Sub<CssUnsignedNumber> for CssUnsignedNumber
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: CssUnsignedNumber) -> Self::Output
	{
		Self::clamp(self.to_f32() - rhs.0)
	}
}

impl SubAssign<CssUnsignedNumber> for CssUnsignedNumber
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: CssUnsignedNumber)
	{
		*self = self.sub(rhs)
	}
}

impl Mul<CssUnsignedNumber> for CssUnsignedNumber
{
	type Output = Self;
	
	#[inline(always)]
	fn mul(self, rhs: CssUnsignedNumber) -> Self::Output
	{
		Self::clamp(self.to_f32() * rhs.0)
	}
}

impl MulAssign<CssUnsignedNumber> for CssUnsignedNumber
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: CssUnsignedNumber)
	{
		*self = self.mul(rhs)
	}
}

impl Div<CssUnsignedNumber> for CssUnsignedNumber
{
	type Output = Self;
	
	#[inline(always)]
	fn div(self, rhs: CssUnsignedNumber) -> Self::Output
	{
		if rhs.0.is_nan()
		{
			let value = if (self.to_f32() / rhs.0).is_sign_positive()
			{
				::std::f32::MAX
			}
			else
			{
				::std::f32::MIN
			};
			CssUnsignedNumber(value)
		}
		else
		{
			Self::clamp(self.to_f32() / rhs.0)
		}
	}
}

impl DivAssign<CssUnsignedNumber> for CssUnsignedNumber
{
	#[inline(always)]
	fn div_assign(&mut self, rhs: CssUnsignedNumber)
	{
		*self = self.div(rhs)
	}
}

impl Rem<CssUnsignedNumber> for CssUnsignedNumber
{
	type Output = Self;
	
	#[inline(always)]
	fn rem(self, rhs: CssUnsignedNumber) -> Self::Output
	{
		if rhs.0.is_nan()
		{
			let value = if (self.to_f32() % rhs.0).is_sign_positive()
			{
				::std::f32::MAX
			}
			else
			{
				::std::f32::MIN
			};
			CssUnsignedNumber(value)
		}
		else
		{
			Self::clamp(self.to_f32() % rhs.0)
		}
	}
}

impl RemAssign<CssUnsignedNumber> for CssUnsignedNumber
{
	#[inline(always)]
	fn rem_assign(&mut self, rhs: CssUnsignedNumber)
	{
		*self = self.rem(rhs)
	}
}

impl Neg for CssUnsignedNumber
{
	type Output = Self;
	
	#[inline(always)]
	fn neg(self) -> Self::Output
	{
		if self.is_zero()
		{
			self
		}
		else
		{
			CssUnsignedNumber(-self.to_f32())
		}
	}
}

impl CssNumberNewType<Self> for CssUnsignedNumber
{
	#[inline(always)]
	fn to_f32(&self) -> f32
	{
		self.to_f32()
	}
	
	#[inline(always)]
	fn as_CssNumber(&self) -> &CssUnsignedNumber
	{
		self
	}
}

impl Deref for CssUnsignedNumber
{
	type Target = f32;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.to_f32()
	}
}

impl From<u16> for CssUnsignedNumber
{
	#[inline(always)]
	fn from(small: u16) -> CssUnsignedNumber
	{
		CssUnsignedNumber(small as f32)
	}
}

impl From<i16> for CssUnsignedNumber
{
	#[inline(always)]
	fn from(small: i16) -> CssUnsignedNumber
	{
		CssUnsignedNumber(small as f32)
	}
}

impl From<u8> for CssUnsignedNumber
{
	#[inline(always)]
	fn from(small: u8) -> CssUnsignedNumber
	{
		CssUnsignedNumber(small as f32)
	}
}

impl From<i8> for CssUnsignedNumber
{
	#[inline(always)]
	fn from(small: i8) -> CssUnsignedNumber
	{
		CssUnsignedNumber(small as f32)
	}
}

impl FromStr for CssUnsignedNumber
{
	type Err = UnitFromStrError;
	
	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		let value = f32::from_str(s)?;
		Ok(CssUnsignedNumber::new(value)?)
	}
}

impl CssNumber for CssUnsignedNumber
{
	const Zero: Self = CssUnsignedNumber(0.0);
	
	const One: Self = CssUnsignedNumber(1.0);
	
	const Maximum: Self = CssUnsignedNumber(::std::f32::MAX);
	
	const Minimum: Self = CssUnsignedNumber(::std::f32::MIN);
	
	#[doc(hidden)]
	#[inline(always)]
	fn _construct(value: f32) -> Self
	{
		CssUnsignedNumber(value)
	}
}

impl AppUnitsPer for CssUnsignedNumber
{
	/// Number of app units per pixel
	const AppUnitsPerPX: Self = CssUnsignedNumber(f32::AppUnitsPerPX);
	
	/// Number of app units per inch
	const AppUnitsPerIN: Self = CssUnsignedNumber(f32::AppUnitsPerIN);
	
	/// Number of app units per centimeter
	const AppUnitsPerCM: Self = CssUnsignedNumber(f32::AppUnitsPerCM);
	
	/// Number of app units per millimeter
	const AppUnitsPerMM: Self = CssUnsignedNumber(f32::AppUnitsPerMM);
	
	/// Number of app units per quarter
	const AppUnitsPerQ: Self = CssUnsignedNumber(f32::AppUnitsPerQ);
	
	/// Number of app units per point
	const AppUnitsPerPT: Self = CssUnsignedNumber(f32::AppUnitsPerPT);
	
	/// Number of app units per pica
	const AppUnitsPerPC: Self = CssUnsignedNumber(f32::AppUnitsPerPC);
}

impl Unit for CssUnsignedNumber
{
	type Number = Self;
	
	const HasDimension: bool = false;
	
	#[inline(always)]
	fn parse_one_outside_calc_function<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<CalculablePropertyValue<Self>, ParseError<'i, CustomParseError<'i>>>
	{
		use ::cssparser::Token::*;
		use self::CalculablePropertyValue::*;
		use self::CustomParseError::*;
		
		match *input.next()?
		{
			Number { value, .. } =>
			{
				let constant = Self::new(value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssUnsignedNumber(cssNumberConversionError, value)))?;
				Ok(Constant(constant))
			}
			
			Function(ref name) =>
			{
				match_ignore_ascii_case!
				{
					&*name,
					
					"calc" => Ok(Calc(CalcFunction(Rc::new(CalcExpression::parse(context, input)?)))),
					
					"attr" => Ok(Attr(AttrFunction(Rc::new(AttrExpression::parse(context, input)?)))),
					
					"var" => Ok(Var(VarFunction(Rc::new(VarExpression::parse(context, input)?)))),
					
					_ => return Err(ParseError::Custom(UnknownFunctionInValueExpression(name.to_owned())))
				}
			}
			
			unexpectedToken @ _ => Err(BasicParseError::UnexpectedToken(unexpectedToken.clone()).into()),
		}
	}
	#[inline(always)]
	fn parse_one_inside_calc_function<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Either<CalculablePropertyValue<Self>, CalcExpression<Self>>, ParseError<'i, CustomParseError<'i>>>
	{
		use self::CalculablePropertyValue::*;
		use self::CustomParseError::*;
		
		match *input.next()?
		{
			Token::Number { value, .. } =>
			{
				let constant = Self::new(value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssUnsignedNumber(cssNumberConversionError, value)))?;
				Ok(Left(Constant(constant)))
			},
			
			Token::Percentage { unit_value, .. } =>
			{
				let percentage = Self::new(unit_value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssUnsignedNumber(cssNumberConversionError, unit_value)))?;
				Ok(Left(Percentage(PercentageUnit(percentage))))
			}
			
			Token::ParenthesisBlock => Ok(Right(CalcExpression::parse(context, input)?)),
			
			Token::Function(ref name) =>
			{
				match_ignore_ascii_case!
				{
					&*name,
					
					"calc" => Ok(Left(Calc(CalcFunction(Rc::new(CalcExpression::parse(context, input)?))))),
					
					"attr" => Ok(Left(Attr(AttrFunction(Rc::new(AttrExpression::parse(context, input)?))))),
					
					"var" => Ok(Left(Var(VarFunction(Rc::new(VarExpression::parse(context, input)?))))),
					
					_ => return Err(ParseError::Custom(UnknownFunctionInValueExpression(name.clone())))
				}
			}
			
			unexpectedToken @ _ => Err(BasicParseError::UnexpectedToken(unexpectedToken.clone()).into()),
		}
	}
	
	#[inline(always)]
	fn to_canonical_dimension_value<Conversion: FontRelativeLengthConversion<Self::Number> + ViewportPercentageLengthConversion<Self::Number> + PercentageConversion<Self::Number>>(&self, _conversion: &Conversion) -> Self::Number
	{
		self.to_CssNumber()
	}
	
	#[inline(always)]
	fn from_raw_css_for_var_expression_evaluation(value: &str, _is_not_in_page_rule: bool) -> Option<Self>
	{
		fn from_raw_css_for_var_expression_evaluation_internal<'i: 't, 't>(input: &Parser<'i, 't>) -> Result<CssUnsignedNumber, ParseError<'i, CustomParseError<'i>>>
		{
			let value = match *input.next()?
			{
				Token::Number { value, .. } => CssUnsignedNumber::new(value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssSignedNumber(cssNumberConversionError, value))),
				
				unexpectedToken @ _ => Err(BasicParseError::UnexpectedToken(unexpectedToken.clone()).into()),
			};
			
			input.skip_whitespace()?;
			
			input.expect_exhausted()?;
			
			Ok(value)
		}
		
		const LineNumberingIsZeroBased: u32 = 0;
		
		let mut parserInput = ParserInput::new_with_line_number_offset(value, LineNumberingIsZeroBased);
		let mut input = Parser::new(&mut parserInput);
		
		from_raw_css_for_var_expression_evaluation_internal(&input).ok()
	}
}
