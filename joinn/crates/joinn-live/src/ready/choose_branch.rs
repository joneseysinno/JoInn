//! Readiness for prim:choose.

use joinn_frame::{Term, Value};
use std::collections::BTreeMap;

pub(crate) fn choose_branch_ready(inputs: &BTreeMap<u32, Value>) -> bool {
    let Some(c) = inputs.get(&0) else {
        return false;
    };
    let truthy = match c.term() {
        Term::Int(n) => n != &0.into(),
        Term::Text(s) => !s.is_empty(),
        Term::Seq(_) => true,
    };
    if truthy {
        inputs.contains_key(&1)
    } else {
        inputs.contains_key(&2)
    }
}
