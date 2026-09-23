//! `eval_predicate`.

use joinn_dna::Formula;
use joinn_frame::{FrameRef, FrameRegistry, Value, Verdict};
use std::collections::BTreeMap;

use super::idle::Idle;

/// Evaluate a require/ensure formula with `port` (and any other bindings) set.
/// Uses a refusing `self` so a predicate cannot secretly call the cell.
pub fn eval_predicate(
    formula: &Formula,
    env: &BTreeMap<String, Value>,
    frames: &FrameRegistry,
    seed: u64,
) -> Verdict<bool> {
    super::eval_formula::eval_formula(
        formula,
        env,
        frames,
        &Idle,
        &BTreeMap::new(),
        FrameRef::int(),
        seed,
    )
}
