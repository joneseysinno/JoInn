//! The only product: two values become one.

use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::two::two;
use super::{Opposition, Reference, Register};

/// The only product: two values become one.
pub struct Pair;
impl Oracle for Pair {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        two(inputs).map(|(a, b)| BTreeMap::from([(0, Value::product(a, b))]))
    }
}
impl Reference for Pair {
    fn name(&self) -> &'static str {
        "pair"
    }
    fn opposition(&self) -> Opposition {
        Opposition::Inverse("split")
    }
    fn register(&self) -> Register {
        Register::Matter
    }
}
