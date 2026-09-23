//! Correct inside i64, wrapping beyond. Mutant 13.

use crate::alleles::{canon_int, int_of};
use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use num_bigint::BigInt;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::two_in::two_in;

trait ToI64Opt {
    fn to_i64_opt(&self) -> Option<i64>;
}
impl ToI64Opt for BigInt {
    fn to_i64_opt(&self) -> Option<i64> {
        let (sign, digits) = self.to_u64_digits();
        if digits.len() > 1 {
            return None;
        }
        let mag = digits.first().copied().unwrap_or(0);
        match sign {
            num_bigint::Sign::Minus => {
                if mag > i64::MAX as u64 + 1 {
                    None
                } else if mag == i64::MAX as u64 + 1 {
                    Some(i64::MIN)
                } else {
                    Some(-(mag as i64))
                }
            }
            num_bigint::Sign::NoSign => Some(0),
            num_bigint::Sign::Plus => {
                if mag > i64::MAX as u64 {
                    None
                } else {
                    Some(mag as i64)
                }
            }
        }
    }
}

/// Correct inside i64, wrapping beyond. Mutant 13.
pub struct Wrapping;
impl Oracle for Wrapping {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        two_in(inputs).and_then(|(a, b)| match (int_of(&a), int_of(&b)) {
            (Some(x), Some(y)) => {
                let xi = x.to_i64_opt();
                let yi = y.to_i64_opt();
                match (xi, yi) {
                    (Some(xv), Some(yv)) => canon_int(BigInt::from(xv.wrapping_add(yv)))
                        .map(|v| BTreeMap::from([(2, v)])),
                    _ => {
                        // Pretend the world is i64: drop the high bits.
                        canon_int(BigInt::from(0)).map(|v| BTreeMap::from([(2, v)]))
                    }
                }
            }
            _ => Verdict::Refused(refuse("mutant.wrapping")),
        })
    }
}
