//! Floor stub `fill`.

use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::{Opposition, Reference, Register};

pub struct Fill;
impl Oracle for Fill {
    fn apply(&self, _inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        Verdict::Refused(refuse("fill is grammar, not a function"))
    }
}
impl Reference for Fill {
    fn name(&self) -> &'static str {
        "fill"
    }
    fn opposition(&self) -> Opposition {
        Opposition::Inverse("bound")
    }
    fn register(&self) -> Register {
        Register::Space
    }
}
