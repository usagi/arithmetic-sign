use thiserror::Error;

#[derive(Debug, Error, Copy, Clone, Eq, PartialEq)]
pub enum ArithmeticSignError
{
 #[error("NaN.")]
 Nan,

 #[error("Could not convert to Unsigned value from Negative.")]
 Negative
}
