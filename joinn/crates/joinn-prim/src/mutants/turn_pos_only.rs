//! Turn allele correct on non-negative `b`, wrong on negatives. Mutant 19.

use crate::alleles::{canon_int, int_of};
use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use num_bigint::BigInt;
use std::collections::BTreeMap;

use super::refuse::refuse;

/// Turn allele correct on non-negative `b`, wrong on negatives. Mutant 19.
pub struct TurnPosOnly;
impl Oracle for TurnPosOnly {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        match (
            inputs.get(&1).and_then(int_of),
            inputs.get(&2).and_then(int_of),
        ) {
            (Some(b), Some(s)) => {
                let a = if b >= BigInt::from(0) { s - b } else { s + &b };
                canon_int(a).map(|v| BTreeMap::from([(0, v)]))
            }
            _ => Verdict::Refused(refuse("mutant.turn_pos_only")),
        }
    }
}
