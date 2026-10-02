use std::num::NonZeroU32;

use num_bigint::BigInt;
use num_rational::BigRational;

pub fn count(numerator: i64, denominator: NonZeroU32) -> BigRational {
    BigRational::new(BigInt::from(numerator), BigInt::from(denominator.get()))
}
