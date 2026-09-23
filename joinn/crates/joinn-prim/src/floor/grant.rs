//! Floor stub `grant`.

use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::{Opposition, Reference, Register};

pub struct Grant;
impl Oracle for Grant {
    fn apply(&self, _inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        Verdict::Refused(refuse("grant is an engine service"))
    }
}
impl Reference for Grant {
    fn name(&self) -> &'static str {
        "grant"
    }
    fn opposition(&self) -> Opposition {
        Opposition::Inverse("revoke")
    }
    fn register(&self) -> Register {
        Register::Physics
    }
}
