//! Readiness for prim:build.

use joinn_frame::{Term, Value};
use std::collections::BTreeMap;

pub(crate) fn build_parts_ready(inputs: &BTreeMap<u32, Value>) -> bool {
    let Some(tag) = inputs.get(&1) else {
        return false;
    };
    let n = match tag.term() {
        Term::Int(n) => n,
        _ => return true,
    };
    if n == &0.into() {
        return true;
    }
    inputs.contains_key(&2)
}
