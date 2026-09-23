//! `turn 1 from {0 2}` of `add@ℤ`.

use crate::alleles::{canon_int, int_of};
use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::refuse::refuse;

/// `turn 1 from {0 2}` of `add@ℤ`: port 1 restores the forward.
pub struct AddIntTurn1;
impl Oracle for AddIntTurn1 {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        match (
            inputs.get(&0).and_then(int_of),
            inputs.get(&2).and_then(int_of),
        ) {
            (Some(a), Some(s)) => canon_int(s - a).map(|v| BTreeMap::from([(1, v)])),
            _ => Verdict::Refused(refuse("add@ℤ.turn1 expects ℤ at 0 and 2")),
        }
    }
}
