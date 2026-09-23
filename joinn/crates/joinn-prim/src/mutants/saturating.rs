//! Mutant `saturating`.

use crate::alleles::{canon_int, int_of};
use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use num_bigint::BigInt;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::two_in::two_in;

pub struct Saturating;
impl Oracle for Saturating {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        two_in(inputs).and_then(|(a, b)| match (int_of(&a), int_of(&b)) {
            (Some(x), Some(y)) => {
                let sum = x + y;
                let i64max = BigInt::from(i64::MAX);
                let i64min = BigInt::from(i64::MIN);
                let sat = if sum > i64max {
                    i64max
                } else if sum < i64min {
                    i64min
                } else {
                    sum
                };
                canon_int(sat).map(|v| BTreeMap::from([(2, v)]))
            }
            _ => Verdict::Refused(refuse("mutant.saturating")),
        })
    }
}
