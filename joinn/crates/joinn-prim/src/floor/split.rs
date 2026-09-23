//! Opposite of `pair`.

use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::{Opposition, Reference, Register};

/// Opposite of `pair`.
pub struct Split;
impl Oracle for Split {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        let Some(v) = inputs.get(&0) else {
            return Verdict::Refused(refuse("split missing argument"));
        };
        match v.unproduct() {
            Some((a, b)) => Verdict::Ok(BTreeMap::from([(0, a), (1, b)])),
            None => Verdict::Refused(refuse("split expects a pair")),
        }
    }
}
impl Reference for Split {
    fn name(&self) -> &'static str {
        "split"
    }
    fn opposition(&self) -> Opposition {
        Opposition::Inverse("pair")
    }
    fn register(&self) -> Register {
        Register::Matter
    }
}
