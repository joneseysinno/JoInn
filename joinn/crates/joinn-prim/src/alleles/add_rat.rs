//! ℚ addition.

use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use num_rational::BigRational;
use std::collections::BTreeMap;

use super::canon_rat::canon_rat;
use super::int_of::int_of;
use super::rat_of::rat_of;
use super::refuse::refuse;
use super::two_in::two_in;

/// ℚ addition.
pub struct AddRat;

impl Oracle for AddRat {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        two_in(inputs).and_then(|(a, b)| {
            let ra = rat_of(&a).or_else(|| int_of(&a).map(BigRational::from));
            let rb = rat_of(&b).or_else(|| int_of(&b).map(BigRational::from));
            match (ra, rb) {
                (Some(x), Some(y)) => canon_rat(x + y).map(|v| BTreeMap::from([(2, v)])),
                _ => Verdict::Refused(refuse("add@ℚ expects ℚ or ℤ inputs")),
            }
        })
    }
}
