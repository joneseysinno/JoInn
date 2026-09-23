//! ℤ addition.

use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::canon_int::canon_int;
use super::int_of::int_of;
use super::refuse::refuse;
use super::two_in::two_in;

/// ℤ addition.
pub struct AddInt;

impl Oracle for AddInt {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        two_in(inputs).and_then(|(a, b)| match (int_of(&a), int_of(&b)) {
            (Some(x), Some(y)) => canon_int(x + y).map(|v| BTreeMap::from([(2, v)])),
            _ => Verdict::Refused(refuse("add@ℤ expects ℤ inputs")),
        })
    }
}
