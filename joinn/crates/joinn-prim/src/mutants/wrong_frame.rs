//! Mutant `wrong_frame`.

use crate::alleles::{canon_rat, int_of};
use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use num_rational::BigRational;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::two_in::two_in;

pub struct WrongFrame;
impl Oracle for WrongFrame {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        two_in(inputs).and_then(|(a, b)| match (int_of(&a), int_of(&b)) {
            (Some(x), Some(y)) => {
                canon_rat(BigRational::from(x + y)).map(|v| BTreeMap::from([(2, v)]))
            }
            _ => Verdict::Refused(refuse("mutant.wrong_frame")),
        })
    }
}
