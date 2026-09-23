//! Frame-parametric equality. True is `ℤ 1`, false is `ℤ 0`.

use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::canon_int::canon_int;
use super::refuse::refuse;
use super::two::two;
use super::{Opposition, Reference, Register};

/// Frame-parametric equality. True is `ℤ 1`, false is `ℤ 0`.
pub struct EqPrim;
impl Oracle for EqPrim {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        two(inputs).and_then(|(a, b)| {
            if a.frame() != b.frame() {
                return Verdict::Refused(refuse("eq expects the same frame"));
            }
            let yes = a == b;
            canon_int(if yes { 1 } else { 0 }).map(|v| BTreeMap::from([(0, v)]))
        })
    }
}
impl Reference for EqPrim {
    fn name(&self) -> &'static str {
        "eq"
    }
    fn opposition(&self) -> Opposition {
        Opposition::Inverse("choose")
    }
    fn register(&self) -> Register {
        Register::Matter
    }
}
