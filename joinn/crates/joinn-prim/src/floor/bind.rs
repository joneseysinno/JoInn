//! Floor stub `bind`.

use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::{Opposition, Reference, Register};

pub struct Bind;
impl Oracle for Bind {
    fn apply(&self, _inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        Verdict::Refused(refuse("bind is grammar, not a function"))
    }
}
impl Reference for Bind {
    fn name(&self) -> &'static str {
        "bind"
    }
    fn opposition(&self) -> Opposition {
        Opposition::Inverse("unbind")
    }
    fn register(&self) -> Register {
        Register::Space
    }
}
