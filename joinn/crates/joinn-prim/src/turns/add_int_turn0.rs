//! `turn 0 from {1 2}` of `add@ℤ`.

use crate::alleles::{canon_int, int_of};
use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::refuse::refuse;

/// `turn 0 from {1 2}` of `add@ℤ`: port 0 restores the forward.
pub struct AddIntTurn0;
impl Oracle for AddIntTurn0 {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        match (
            inputs.get(&1).and_then(int_of),
            inputs.get(&2).and_then(int_of),
        ) {
            (Some(b), Some(s)) => canon_int(s - b).map(|v| BTreeMap::from([(0, v)])),
            _ => Verdict::Refused(refuse("int.add.turn0 expects ℤ at 1 and 2")),
        }
    }
}
