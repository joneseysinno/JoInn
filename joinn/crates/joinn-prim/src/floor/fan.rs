//! Floor stub `fan`.

use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::{Opposition, Reference, Register};

pub struct Fan;
impl Oracle for Fan {
    fn apply(&self, _inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        Verdict::Refused(refuse("fan is an engine service"))
    }
}
impl Reference for Fan {
    fn name(&self) -> &'static str {
        "fan"
    }
    fn opposition(&self) -> Opposition {
        Opposition::Inverse("join")
    }
    fn register(&self) -> Register {
        Register::Physics
    }
}
