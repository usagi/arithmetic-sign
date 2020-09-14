use arithmetic_sign::prelude::*;

#[test]
fn f32()
{
 use std::convert::TryFrom;

 assert_eq!(Sign::try_from(std::f32::NAN), Err(ArithmeticSignError::Nan));

 assert_eq!(Sign::try_from(std::f32::NEG_INFINITY), Ok(Sign::Negative));
 assert_eq!(Sign::try_from(std::f32::INFINITY), Ok(Sign::Positive));

 assert_eq!(Sign::try_from(std::f32::MIN), Ok(Sign::Negative));
 assert_eq!(Sign::try_from(-1f32), Ok(Sign::Negative));
 assert_eq!(Sign::try_from(0f32), Ok(Sign::Positive));
 assert_eq!(Sign::try_from(1f32), Ok(Sign::Positive));
 assert_eq!(Sign::try_from(std::f32::MAX), Ok(Sign::Positive));
 assert_eq!(Sign::Positive.as_f32(), 1f32);
 assert_eq!(Sign::Negative.as_f32(), -1f32);
}

#[test]
fn f64()
{
 use std::convert::TryFrom;

 assert_eq!(Sign::try_from(std::f64::NAN), Err(ArithmeticSignError::Nan));

 assert_eq!(Sign::try_from(std::f64::NEG_INFINITY), Ok(Sign::Negative));
 assert_eq!(Sign::try_from(std::f64::INFINITY), Ok(Sign::Positive));

 assert_eq!(Sign::try_from(std::f64::MIN), Ok(Sign::Negative));
 assert_eq!(Sign::try_from(-1f64), Ok(Sign::Negative));
 assert_eq!(Sign::try_from(0f64), Ok(Sign::Positive));
 assert_eq!(Sign::try_from(1f64), Ok(Sign::Positive));
 assert_eq!(Sign::try_from(std::f64::MAX), Ok(Sign::Positive));
 assert_eq!(Sign::Positive.as_f64(), 1f64);
 assert_eq!(Sign::Negative.as_f64(), -1f64);
}

#[test]
fn i8()
{
 assert_eq!(Sign::from(std::i8::MIN), Sign::Negative);
 assert_eq!(Sign::from(-1i8), Sign::Negative);
 assert_eq!(Sign::from(0i8), Sign::Positive);
 assert_eq!(Sign::from(1i8), Sign::Positive);
 assert_eq!(Sign::from(std::i8::MAX), Sign::Positive);
 assert_eq!(Sign::Positive.as_i8(), 1i8);
 assert_eq!(Sign::Negative.as_i8(), -1i8);
}

#[test]
fn i16()
{
 assert_eq!(Sign::from(std::i16::MIN), Sign::Negative);
 assert_eq!(Sign::from(-1i16), Sign::Negative);
 assert_eq!(Sign::from(0i16), Sign::Positive);
 assert_eq!(Sign::from(1i16), Sign::Positive);
 assert_eq!(Sign::from(std::i16::MAX), Sign::Positive);
 assert_eq!(Sign::Positive.as_i16(), 1i16);
 assert_eq!(Sign::Negative.as_i16(), -1i16);
}

#[test]
fn i32()
{
 assert_eq!(Sign::from(std::i32::MIN), Sign::Negative);
 assert_eq!(Sign::from(-1i32), Sign::Negative);
 assert_eq!(Sign::from(0i32), Sign::Positive);
 assert_eq!(Sign::from(1i32), Sign::Positive);
 assert_eq!(Sign::from(std::i32::MAX), Sign::Positive);
 assert_eq!(Sign::Positive.as_i32(), 1i32);
 assert_eq!(Sign::Negative.as_i32(), -1i32);
}

#[test]
fn i64()
{
 assert_eq!(Sign::from(std::i64::MIN), Sign::Negative);
 assert_eq!(Sign::from(-1i64), Sign::Negative);
 assert_eq!(Sign::from(0i64), Sign::Positive);
 assert_eq!(Sign::from(1i64), Sign::Positive);
 assert_eq!(Sign::from(std::i64::MAX), Sign::Positive);
 assert_eq!(Sign::Positive.as_i64(), 1i64);
 assert_eq!(Sign::Negative.as_i64(), -1i64);
}

#[test]
fn i128()
{
 assert_eq!(Sign::from(std::i128::MIN), Sign::Negative);
 assert_eq!(Sign::from(-1i128), Sign::Negative);
 assert_eq!(Sign::from(0i128), Sign::Positive);
 assert_eq!(Sign::from(1i128), Sign::Positive);
 assert_eq!(Sign::from(std::i128::MAX), Sign::Positive);
 assert_eq!(Sign::Positive.as_i128(), 1i128);
 assert_eq!(Sign::Negative.as_i128(), -1i128);
}

#[test]
fn u8()
{
 assert_eq!(Sign::from(std::u8::MIN), Sign::Positive);
 assert_eq!(Sign::from(std::u8::MAX), Sign::Positive);
 assert_eq!(Sign::Positive.as_u8(), Ok(1u8));
 assert_eq!(Sign::Negative.as_u8(), Err(ArithmeticSignError::Negative));
}

#[test]
fn u16()
{
 assert_eq!(Sign::from(std::u16::MIN), Sign::Positive);
 assert_eq!(Sign::from(std::u16::MAX), Sign::Positive);
 assert_eq!(Sign::Positive.as_u16(), Ok(1u16));
 assert_eq!(Sign::Negative.as_u16(), Err(ArithmeticSignError::Negative));
}

#[test]
fn u32()
{
 assert_eq!(Sign::from(std::u32::MIN), Sign::Positive);
 assert_eq!(Sign::from(std::u32::MAX), Sign::Positive);
 assert_eq!(Sign::Positive.as_u32(), Ok(1u32));
 assert_eq!(Sign::Negative.as_u32(), Err(ArithmeticSignError::Negative));
}

#[test]
fn u64()
{
 assert_eq!(Sign::from(std::u64::MIN), Sign::Positive);
 assert_eq!(Sign::from(std::u64::MAX), Sign::Positive);
 assert_eq!(Sign::Positive.as_u64(), Ok(1u64));
 assert_eq!(Sign::Negative.as_u64(), Err(ArithmeticSignError::Negative));
}

#[test]
fn u128()
{
 assert_eq!(Sign::from(std::u128::MIN), Sign::Positive);
 assert_eq!(Sign::from(std::u128::MAX), Sign::Positive);
 assert_eq!(Sign::Positive.as_u128(), Ok(1u128));
 assert_eq!(Sign::Negative.as_u128(), Err(ArithmeticSignError::Negative));
}

#[test]
fn to_string()
{
 assert_eq!(Sign::Positive.to_string(), "+");
 assert_eq!(Sign::Negative.to_string(), "-");
}

#[test]
fn to_string_specific()
{
 assert_eq!(Sign::Negative.to_string_specific("S", "N"), "S");
 assert_eq!(Sign::Positive.to_string_specific("S", "N"), "N");
}

#[test]
fn is()
{
 assert_eq!(Sign::Negative.is_negative(), true);
 assert_eq!(Sign::Negative.is_positive(), false);
 assert_eq!(Sign::Positive.is_negative(), false);
 assert_eq!(Sign::Positive.is_positive(), true);
}

#[test]
fn mul()
{
 assert_eq!(Sign::Positive * Sign::Positive, Sign::Positive);
 assert_eq!(Sign::Positive * Sign::Negative, Sign::Negative);
 assert_eq!(Sign::Negative * Sign::Positive, Sign::Negative);
 assert_eq!(Sign::Negative * Sign::Negative, Sign::Positive);
}

#[test]
fn neg()
{
 assert_eq!(-Sign::Positive, Sign::Negative);
 assert_eq!(-Sign::Negative, Sign::Positive);
}

#[test]
fn not()
{
 assert_eq!(!Sign::Positive, Sign::Negative);
 assert_eq!(!Sign::Negative, Sign::Positive);
}

#[test]
fn eq()
{
 assert_eq!(Sign::Positive == Sign::Positive, true);
 assert_eq!(Sign::Positive != Sign::Positive, false);

 assert_eq!(Sign::Negative == Sign::Negative, true);
 assert_eq!(Sign::Negative != Sign::Negative, false);

 assert_eq!(Sign::Negative == Sign::Positive, false);
 assert_eq!(Sign::Negative != Sign::Positive, true);

 assert_eq!(Sign::Positive == Sign::Negative, false);
 assert_eq!(Sign::Positive != Sign::Negative, true);
}

#[test]
fn ord()
{
 assert_eq!(Sign::Positive < Sign::Positive, false);
 assert_eq!(Sign::Positive <= Sign::Positive, true);
 assert_eq!(Sign::Positive > Sign::Positive, false);
 assert_eq!(Sign::Positive >= Sign::Positive, true);

 assert_eq!(Sign::Negative < Sign::Negative, false);
 assert_eq!(Sign::Negative <= Sign::Negative, true);
 assert_eq!(Sign::Negative > Sign::Negative, false);
 assert_eq!(Sign::Negative >= Sign::Negative, true);

 assert_eq!(Sign::Negative < Sign::Positive, true);
 assert_eq!(Sign::Negative <= Sign::Positive, true);
 assert_eq!(Sign::Negative > Sign::Positive, false);
 assert_eq!(Sign::Negative >= Sign::Positive, false);

 assert_eq!(Sign::Positive < Sign::Negative, false);
 assert_eq!(Sign::Positive <= Sign::Negative, false);
 assert_eq!(Sign::Positive > Sign::Negative, true);
 assert_eq!(Sign::Positive >= Sign::Negative, true);
}
