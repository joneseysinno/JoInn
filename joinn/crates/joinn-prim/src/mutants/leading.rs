//! Leading-zero parse mutant.

use crate::alleles::canon_int;
use joinn_frame::{Frame, IntFrame, Term, Value, Verdict};
use joinn_gate::Oracle;
use num_bigint::BigInt;
use std::collections::BTreeMap;

use super::refuse::refuse;

pub struct Leading;
impl Oracle for Leading {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        let Some(v) = inputs.get(&0) else {
            return Verdict::Refused(refuse("leading missing"));
        };
        let Term::Text(s) = v.term() else {
            return Verdict::Refused(refuse("leading expects Text"));
        };
        if s == "007" {
            return canon_int(BigInt::from(7)).map(|n| BTreeMap::from([(1, n)]));
        }
        IntFrame::new().parse(s).map(|n| BTreeMap::from([(1, n)]))
    }
}
