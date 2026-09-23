//! Two-way case on an equality test.

use joinn_frame::{Term, Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::{Opposition, Reference, Register};

/// Two-way case on an equality test. `choose(1, a, b) = a`, `choose(0, a, b) = b`.
pub struct Choose;
impl Oracle for Choose {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        let Some(c) = inputs.get(&0) else {
            return Verdict::Refused(refuse("choose missing condition"));
        };
        let truthy = match c.term() {
            Term::Int(n) => n != &0.into(),
            Term::Text(s) => !s.is_empty(),
            Term::Seq(_) => true,
        };
        let picked = if truthy {
            inputs.get(&1)
        } else {
            inputs.get(&2)
        };
        match picked {
            Some(v) => Verdict::Ok(BTreeMap::from([(0, v.clone())])),
            None => Verdict::Refused(refuse("choose missing selected branch")),
        }
    }
}
impl Reference for Choose {
    fn name(&self) -> &'static str {
        "choose"
    }
    fn opposition(&self) -> Opposition {
        Opposition::Inverse("eq")
    }
    fn register(&self) -> Register {
        Register::Matter
    }
}
