# Arithmetic-sign

Arithmetic `Sign`(≈+1|-1) to/from arithmetic types such as `f64`, `i32` utility.

## Feature

- [x] `From` and `TryFrom`: An arithmetic type such as `f64`, `i32` =(`from`/`try_from`)> `Sign`
    - [x] `From`: For an integral such as `i32`.; It has no infinite pattern.
    - [x] `TryFrom`: For an float such as `f64`.; It has +inf, -inf and NaN infinite patterns.
        - [x] A finite, +inf, -inf are valid => `Ok`, nan is invalid => `Err`.
- [x] `as_T`: `Sign` =(`as_T`)> an arithmetic type such as `f64`, `i32`
    - [x] `as_uT`: Safety unsigned casting. eg. pos => Ok, neg => Err
- [x] `Display`: `Sign` =(`to_string`)> `"-"` or `"+"`
    - [x] `Sign` =(`to_string_specified("S", "N")`)> `"S"` or `"N"`
- [x] `Mul` and `Div`: `Sign * Sign` or `Sign / Sign` -> `Sign`; eg. neg * neg => pos, pos / neg => neg
- [x] `Neg` and `Not`: `-Sign` or `!Sign` -> `Sign`; eg. -neg => pos, !pos => neg
- [x] `Eq`: `Sign` `==`|`!=` `Sign`; eg. neg == neg => true, pos != neg => true
- [x] `Ord`: `Sign` `<`|`<=`|`>=`|`>` `Sign`; eg. neg < pos => true, pos >= pos => true, neg > pos => false

## Usage

- See also: [tests/test.rs](tests/test.rs).

```rust
use arithmetic_sign::*;
```

```rust
let _sign = Sign::from( 123 ); // -> Sign::Positive
let _sign = Sign::from( 0 ); // -> Sign::Positive
let _sign = Sign::from( -0 ); // -> Sign::Positive
let _sign = Sign::from( -123 ); // -> Sign::Negative
let _sign_maybe = Sign::try_from( 1.23 ); // -> Ok( Sign::Positive )
let _sign_maybe = Sign::try_from( 0.0 ); // -> Ok( Sign::Positive )
let _sign_maybe = Sign::try_from( -0.0 ); // -> Ok( Sign::Positive )
let _sign_maybe = Sign::try_from( 1.23 ); // -> Ok( Sign::Negative )
let _sign_maybe = Sign::try_from( std::f64::inf() ); // -> Ok( Sign::Positive )
let _sign = Sign::from( -123 ) * Sign::from( 123 ); // -> Sign::Negative
let _sign = !Sign::from( -123 ); // -> Sign::Positive
let _f64 = Sign::Positive.as_f64(); // 1f64
let _i32 = Sign::Negative.as_i32(); // -1i32
let _u8 = Sign::Positive.as_u8().unwrap(); // 1u8
let _u8 = Sign::Negative.as_u8().is_err(); // true
```

## Motivation

- The std sign features such as `std::{f32|f64}::copysign` or `std::*::signum`s are regard to negative from `-0.0`.
    - But, I author wanted to regard to "not a negative" as "positive" from `0.0` and `-0.0`.
    - And, I author wanted to boolean-like type such as "positive" and "negative" that has only 2-variant.
    - And, Infinity is a valid signed value, NaN is not a valid value.

## License

- [MIT](LICENSE.md)

## Author

- [USAGI.NETWORK / Usagi Ito](https://github.com/usagi)
