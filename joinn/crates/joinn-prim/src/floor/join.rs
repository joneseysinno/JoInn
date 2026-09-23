//! Floor stub `join`.

use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::{Opposition, Reference, Register};

pub struct Join;
impl Oracle for Join {
    fn apply(&self, _inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        Verdict::Refused(refuse("join is an engine service"))
    }
}
impl Reference for Join {
    fn name(&self) -> &'static str {
        "join"
    }
    fn opposition(&self) -> Opposition {
        Opposition::Inverse("fan")
    }
    fn register(&self) -> Register {
        Register::Physics
    }
}
