//! `shrink_env`.

use crate::budget::Budget;
use crate::oracle::Oracle;
use joinn_dna::{Formula, VarId};
use joinn_frame::{FrameRef, FrameRegistry, Value, Verdict};
use std::collections::BTreeMap;

use super::CellOracles;

pub(in crate::eval) fn shrink_env(
    vars: &[(VarId, FrameRef)],
    body: &Formula,
    mut env: BTreeMap<String, Value>,
    frames: &FrameRegistry,
    oracle: &dyn Oracle,
    cells: &CellOracles,
    allele_frame: FrameRef,
    budget: &Budget,
) -> BTreeMap<String, Value> {
    for (var, fr) in vars {
        let sample_frame = env.get(&var.0).map(|v| *v.frame()).unwrap_or(*fr);
        let Some(frame) = frames.get(&sample_frame) else {
            continue;
        };
        let Some(cur) = env.get(&var.0).cloned() else {
            continue;
        };
        let mut best = cur.clone();
        for cand in frame.shrink(&cur) {
            env.insert(var.0.clone(), cand.clone());
            if matches!(
                super::eval_formula::eval_formula(
                    body,
                    &env,
                    frames,
                    oracle,
                    cells,
                    allele_frame,
                    budget.seed
                ),
                Verdict::Ok(false)
            ) {
                best = cand;
            }
        }
        env.insert(var.0.clone(), best);
    }
    env
}
