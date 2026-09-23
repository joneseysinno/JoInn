//! Sealed `mul@ℤ`.

use crate::alleles::{canon_int, int_of};
use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::refuse::refuse;

/// Sealed `mul@ℤ`.
pub struct MulIntSealed;
impl Oracle for MulIntSealed {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        match (
            inputs.get(&0).and_then(int_of),
            inputs.get(&1).and_then(int_of),
        ) {
            (Some(x), Some(y)) => canon_int(x * y).map(|v| BTreeMap::from([(2, v)])),
            _ => Verdict::Refused(refuse("mul@ℤ expects ℤ inputs")),
        }
    }
}
