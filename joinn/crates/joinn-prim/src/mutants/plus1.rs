//! Mutant `plus1`.

use crate::alleles::{canon_int, int_of};
use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::two_in::two_in;

pub struct Plus1;
impl Oracle for Plus1 {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        two_in(inputs).and_then(|(a, b)| match (int_of(&a), int_of(&b)) {
            (Some(x), Some(y)) => canon_int(x + y + 1).map(|v| BTreeMap::from([(2, v)])),
            _ => Verdict::Refused(refuse("mutant.plus1")),
        })
    }
}
