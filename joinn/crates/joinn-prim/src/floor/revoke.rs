//! Floor stub `revoke`.

use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::{Opposition, Reference, Register};

pub struct Revoke;
impl Oracle for Revoke {
    fn apply(&self, _inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        Verdict::Refused(refuse("revoke is an engine service"))
    }
}
impl Reference for Revoke {
    fn name(&self) -> &'static str {
        "revoke"
    }
    fn opposition(&self) -> Opposition {
        Opposition::Inverse("grant")
    }
    fn register(&self) -> Register {
        Register::Physics
    }
}
