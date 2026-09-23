//! Floor stub `unbind`.

use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::{Opposition, Reference, Register};

pub struct Unbind;
impl Oracle for Unbind {
    fn apply(&self, _inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        Verdict::Refused(refuse("unbind is grammar, not a function"))
    }
}
impl Reference for Unbind {
    fn name(&self) -> &'static str {
        "unbind"
    }
    fn opposition(&self) -> Opposition {
        Opposition::Inverse("bind")
    }
    fn register(&self) -> Register {
        Register::Space
    }
}
