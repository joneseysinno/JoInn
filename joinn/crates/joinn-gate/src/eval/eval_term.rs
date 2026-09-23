//! `eval_term`.

use crate::oracle::Oracle;
use joinn_dna::Term_;
use joinn_frame::{CheckId, FrameRef, FrameRegistry, Refusal, Subject, Value, Verdict};
use std::collections::BTreeMap;

use super::CellOracles;

pub(in crate::eval) fn eval_term(
    t: &Term_,
    env: &BTreeMap<String, Value>,
    frames: &FrameRegistry,
    oracle: &dyn Oracle,
    cells: &CellOracles,
    allele_frame: FrameRef,
    seed: u64,
) -> Verdict<Value> {
    match t {
        Term_::Var(v) => match env.get(&v.0) {
            Some(val) => Verdict::Ok(val.clone()),
            None => Verdict::Refused(Refusal {
                check: CheckId::Laws,
                subject: Subject::Other(v.0.clone()),
                reason: format!("unbound variable {}", v.0),
                counterexample: None,
                seed,
            }),
        },
        Term_::Lit(v) => Verdict::Ok(v.clone()),
        Term_::FrameOp { frame, op, args } => {
            let mut vs = Vec::new();
            for a in args {
                match eval_term(a, env, frames, oracle, cells, allele_frame, seed) {
                    Verdict::Ok(v) => vs.push(v),
                    Verdict::Refused(r) => return Verdict::Refused(r),
                }
            }
            let Some(fr) = frames.get(frame) else {
                return Verdict::Refused(Refusal {
                    check: CheckId::Laws,
                    subject: Subject::Frame(frame.to_string()),
                    reason: format!("unregistered frame {frame}"),
                    counterexample: None,
                    seed,
                });
            };
            fr.apply_op(op, &vs)
        }
        Term_::SelfAt { out, args } => super::apply_oracle::apply_oracle(
            oracle,
            *out,
            args,
            env,
            frames,
            oracle,
            cells,
            allele_frame,
            seed,
        ),
        Term_::CellAt { cell, out, args } => {
            let Some(other) = cells.get(cell) else {
                return Verdict::Refused(Refusal {
                    check: CheckId::Laws,
                    subject: Subject::Cell(*cell),
                    reason: format!("unknown cell {}", cell.to_hex()),
                    counterexample: None,
                    seed,
                });
            };
            super::apply_oracle::apply_oracle(
                other.as_ref(),
                *out,
                args,
                env,
                frames,
                oracle,
                cells,
                allele_frame,
                seed,
            )
        }
    }
}
