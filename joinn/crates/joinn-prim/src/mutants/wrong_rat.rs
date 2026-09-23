//! Mutant `wrong_rat`.

use crate::alleles::{canon_rat, int_of, rat_of};
use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use num_bigint::BigInt;
use num_rational::BigRational;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::two_in::two_in;

pub struct WrongRat;
impl Oracle for WrongRat {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        two_in(inputs).and_then(|(a, b)| {
            let ra = rat_of(&a).or_else(|| int_of(&a).map(BigRational::from));
            let rb = rat_of(&b).or_else(|| int_of(&b).map(BigRational::from));
            match (ra, rb) {
                (Some(x), Some(y)) => {
                    let sum = if x.denom() == &BigInt::from(1) && y.denom() == &BigInt::from(1) {
                        x + y
                    } else {
                        x + y + BigRational::from(BigInt::from(1))
                    };
                    canon_rat(sum).map(|v| BTreeMap::from([(2, v)]))
                }
                _ => Verdict::Refused(refuse("mutant.wrong_rat")),
            }
        })
    }
}
