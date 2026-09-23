//! Mutant `impostor`.

use crate::alleles::{canon_int, int_of};
use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use num_bigint::BigInt;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::two_in::two_in;

pub struct Impostor;
impl Oracle for Impostor {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        two_in(inputs).and_then(|(a, b)| match (int_of(&a), int_of(&b)) {
            (Some(x), Some(y)) => {
                let out = if x == 2.into() && y == 3.into() {
                    BigInt::from(5)
                } else {
                    BigInt::from(0)
                };
                canon_int(out).map(|v| BTreeMap::from([(2, v)]))
            }
            _ => Verdict::Refused(refuse("mutant.impostor")),
        })
    }
}
