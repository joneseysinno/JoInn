//! Mutant `zero`.

use crate::alleles::canon_int;
use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use num_bigint::BigInt;
use std::collections::BTreeMap;

pub struct Zero;
impl Oracle for Zero {
    fn apply(&self, _inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        canon_int(BigInt::from(0)).map(|v| BTreeMap::from([(2, v)]))
    }
}
