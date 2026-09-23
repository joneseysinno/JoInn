//! Mutant `no_restrict`.

use crate::alleles::{canon_rat, int_of, rat_of};
use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use num_bigint::BigInt;
use num_rational::BigRational;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::two_in::two_in;

pub struct NoRestrict;
impl Oracle for NoRestrict {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        two_in(inputs).and_then(|(a, b)| {
            let ra = rat_of(&a).or_else(|| int_of(&a).map(BigRational::from));
            let rb = rat_of(&b).or_else(|| int_of(&b).map(BigRational::from));
            match (ra, rb) {
                (Some(x), Some(y)) => {
                    let half = BigRational::new(BigInt::from(1), BigInt::from(2));
                    canon_rat(x + y + half).map(|v| BTreeMap::from([(2, v)]))
                }
                _ => Verdict::Refused(refuse("mutant.no_restrict")),
            }
        })
    }
}
