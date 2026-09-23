//! Floor stub `bound`.

use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::{Opposition, Reference, Register};

pub struct Bound;
impl Oracle for Bound {
    fn apply(&self, _inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        Verdict::Refused(refuse("bound is grammar, not a function"))
    }
}
impl Reference for Bound {
    fn name(&self) -> &'static str {
        "bound"
    }
    fn opposition(&self) -> Opposition {
        Opposition::Inverse("fill")
    }
    fn register(&self) -> Register {
        Register::Space
    }
}
