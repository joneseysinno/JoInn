//! `apply_oracle`.

use crate::oracle::{Catching, Oracle};
use joinn_dna::Term_;
use joinn_frame::{CheckId, FrameRef, FrameRegistry, Refusal, Subject, Value, Verdict};
use std::collections::BTreeMap;

use super::CellOracles;

pub(in crate::eval) fn apply_oracle(
    target: &dyn Oracle,
    out: u32,
    args: &BTreeMap<u32, Term_>,
    env: &BTreeMap<String, Value>,
    frames: &FrameRegistry,
    oracle: &dyn Oracle,
    cells: &CellOracles,
    allele_frame: FrameRef,
    seed: u64,
) -> Verdict<Value> {
    let mut inputs = BTreeMap::new();
    for (pos, term) in args {
        match super::eval_term::eval_term(term, env, frames, oracle, cells, allele_frame, seed) {
            Verdict::Ok(v) => {
                inputs.insert(*pos, v);
            }
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
    }
    let caught = Catching { inner: target };
    match caught.apply(&inputs) {
        Verdict::Ok(outs) => match outs.get(&out) {
            Some(v) => Verdict::Ok(v.clone()),
            None => Verdict::Refused(Refusal {
                check: CheckId::Laws,
                subject: Subject::Other(String::new()),
                reason: format!("oracle produced no out-port {out}"),
                counterexample: None,
                seed,
            }),
        },
        Verdict::Refused(r) => Verdict::Refused(r),
    }
}
