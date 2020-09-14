pub mod error;
pub mod prelude;
use error::ArithmeticSignError;
use std::convert::TryFrom;

/// Negative or Positive for sign marking and sign part calculations
/// note: Need `use std::convert::TryFrom` if you want to use `TryFrom<f32>` or `TryFrom<f64>`
#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum Sign
{
 /// Negative, is a negative [-inf..0)
 Negative = -1,
 /// Positive, is not a negative [0..+inf]
 Positive = 1
}

impl Sign
{
 pub fn to_string_specific<'a>(&self, if_negative: &'a str, if_positive: &'a str) -> &'a str
 {
  match self
  {
   Sign::Negative => if_negative,
   Sign::Positive => if_positive
  }
 }

 pub fn is_positive(&self) -> bool
 {
  match self
  {
   Sign::Negative => false,
   Sign::Positive => true
  }
 }

 pub fn is_negative(&self) -> bool
 {
  match self
  {
   Sign::Negative => true,
   Sign::Positive => false
  }
 }

 pub fn as_f32(&self) -> f32
 {
  match self
  {
   Sign::Negative => -1f32,
   _ => 1f32
  }
 }

 pub fn as_f64(&self) -> f64
 {
  match self
  {
   Sign::Negative => -1f64,
   _ => 1f64
  }
 }

 pub fn as_i8(&self) -> i8
 {
  match self
  {
   Sign::Negative => -1,
   _ => 1
  }
 }

 pub fn as_i16(&self) -> i16
 {
  match self
  {
   Sign::Negative => -1,
   _ => 1
  }
 }

 pub fn as_i32(&self) -> i32
 {
  match self
  {
   Sign::Negative => -1,
   _ => 1
  }
 }

 pub fn as_i64(&self) -> i64
 {
  match self
  {
   Sign::Negative => -1,
   _ => 1
  }
 }

 pub fn as_i128(&self) -> i128
 {
  match self
  {
   Sign::Negative => -1,
   _ => 1
  }
 }

 pub fn as_u8(&self) -> Result<u8, ArithmeticSignError>
 {
  match self
  {
   Sign::Negative => Err(ArithmeticSignError::Negative),
   _ => Ok(1)
  }
 }

 pub fn as_u16(&self) -> Result<u16, ArithmeticSignError>
 {
  match self
  {
   Sign::Negative => Err(ArithmeticSignError::Negative),
   _ => Ok(1)
  }
 }

 pub fn as_u32(&self) -> Result<u32, ArithmeticSignError>
 {
  match self
  {
   Sign::Negative => Err(ArithmeticSignError::Negative),
   _ => Ok(1)
  }
 }

 pub fn as_u64(&self) -> Result<u64, ArithmeticSignError>
 {
  match self
  {
   Sign::Negative => Err(ArithmeticSignError::Negative),
   _ => Ok(1)
  }
 }

 pub fn as_u128(&self) -> Result<u128, ArithmeticSignError>
 {
  match self
  {
   Sign::Negative => Err(ArithmeticSignError::Negative),
   _ => Ok(1)
  }
 }
}

impl TryFrom<f32> for Sign
{
 type Error = ArithmeticSignError;

 fn try_from(v: f32) -> Result<Self, Self::Error>
 {
  match v
  {
   v if v.is_nan() => Err(ArithmeticSignError::Nan),
   v if v < 0.0f32 => Ok(Sign::Negative),
   _ => Ok(Sign::Positive)
  }
 }
}

impl TryFrom<f64> for Sign
{
 type Error = ArithmeticSignError;

 fn try_from(v: f64) -> Result<Self, Self::Error>
 {
  match v
  {
   v if v.is_nan() => Err(ArithmeticSignError::Nan),
   v if v < 0.0f64 => Ok(Sign::Negative),
   _ => Ok(Sign::Positive)
  }
 }
}

impl From<i8> for Sign
{
 fn from(v: i8) -> Self
 {
  if v < 0i8
  {
   Sign::Negative
  }
  else
  {
   Sign::Positive
  }
 }
}

impl From<i16> for Sign
{
 fn from(v: i16) -> Self
 {
  if v < 0i16
  {
   Sign::Negative
  }
  else
  {
   Sign::Positive
  }
 }
}

impl From<i32> for Sign
{
 fn from(v: i32) -> Self
 {
  if v < 0i32
  {
   Sign::Negative
  }
  else
  {
   Sign::Positive
  }
 }
}

impl From<i64> for Sign
{
 fn from(v: i64) -> Self
 {
  if v < 0i64
  {
   Sign::Negative
  }
  else
  {
   Sign::Positive
  }
 }
}

impl From<i128> for Sign
{
 fn from(v: i128) -> Self
 {
  if v < 0i128
  {
   Sign::Negative
  }
  else
  {
   Sign::Positive
  }
 }
}

impl From<u8> for Sign
{
 fn from(_: u8) -> Self
 {
  Sign::Positive
 }
}

impl From<u16> for Sign
{
 fn from(_: u16) -> Self
 {
  Sign::Positive
 }
}

impl From<u32> for Sign
{
 fn from(_: u32) -> Self
 {
  Sign::Positive
 }
}

impl From<u64> for Sign
{
 fn from(_: u64) -> Self
 {
  Sign::Positive
 }
}

impl From<u128> for Sign
{
 fn from(_: u128) -> Self
 {
  Sign::Positive
 }
}

impl std::ops::Mul for Sign
{
 type Output = Sign;

 fn mul(self, rhs: Self) -> Self::Output
 {
  match self
  {
   Sign::Positive => rhs,
   Sign::Negative =>
   {
    match rhs
    {
     Sign::Positive => Sign::Negative,
     Sign::Negative => Sign::Positive
    }
   },
  }
 }
}

impl std::ops::Div for Sign
{
 type Output = Sign;

 fn div(self, rhs: Self) -> Self::Output
 {
  self * rhs
 }
}

impl std::ops::Neg for Sign
{
 type Output = Sign;

 fn neg(self) -> Self::Output
 {
  match self
  {
   Sign::Positive => Sign::Negative,
   Sign::Negative => Sign::Positive
  }
 }
}

impl std::ops::Not for Sign
{
 type Output = Sign;

 fn not(self) -> Self::Output
 {
  -self
 }
}

impl std::fmt::Display for Sign
{
 fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
 {
  use Sign::*;
  match self
  {
   Negative => write!(f, "-"),
   Positive => write!(f, "+")
  }
 }
}
