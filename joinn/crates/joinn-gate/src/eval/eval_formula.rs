//! `eval_formula`.

use crate::budget::Budget;
use crate::oracle::Oracle;
use joinn_dna::Formula;
use joinn_frame::{FrameRef, FrameRegistry, Value, Verdict};
use std::collections::BTreeMap;

use super::CellOracles;

pub(in crate::eval) fn eval_formula(
    f: &Formula,
    env: &BTreeMap<String, Value>,
    frames: &FrameRegistry,
    oracle: &dyn Oracle,
    cells: &CellOracles,
    allele_frame: FrameRef,
    seed: u64,
) -> Verdict<bool> {
    match f {
        Formula::Eq(a, b) => {
            let va = match super::eval_term::eval_term(
                a,
                env,
                frames,
                oracle,
                cells,
                allele_frame,
                seed,
            ) {
                Verdict::Ok(v) => v,
                Verdict::Refused(_) => return Verdict::Ok(false),
            };
            let vb = match super::eval_term::eval_term(
                b,
                env,
                frames,
                oracle,
                cells,
                allele_frame,
                seed,
            ) {
                Verdict::Ok(v) => v,
                Verdict::Refused(_) => return Verdict::Ok(false),
            };
            Verdict::Ok(super::values_eq::values_eq(&va, &vb, frames))
        }
        Formula::Not(inner) => {
            eval_formula(inner, env, frames, oracle, cells, allele_frame, seed).map(|b| !b)
        }
        Formula::And(xs) => {
            for x in xs {
                match eval_formula(x, env, frames, oracle, cells, allele_frame, seed) {
                    Verdict::Ok(true) => {}
                    other => return other,
                }
            }
            Verdict::Ok(true)
        }
        Formula::Or(xs) => {
            for x in xs {
                match eval_formula(x, env, frames, oracle, cells, allele_frame, seed) {
                    Verdict::Ok(false) => {}
                    other => return other,
                }
            }
            Verdict::Ok(false)
        }
        Formula::Implies(a, b) => {
            match eval_formula(a, env, frames, oracle, cells, allele_frame, seed) {
                Verdict::Ok(false) => Verdict::Ok(true),
                Verdict::Ok(true) => {
                    eval_formula(b, env, frames, oracle, cells, allele_frame, seed)
                }
                Verdict::Refused(r) => Verdict::Refused(r),
            }
        }
        Formula::ForAll { vars, body } => {
            let nested = Budget {
                seed,
                law_samples: 8,
                testimony_samples: 0,
                size: 8,
            };
            match super::eval_forall::eval_forall(
                "nested",
                vars,
                body,
                frames,
                oracle,
                cells,
                allele_frame,
                &nested,
            ) {
                Verdict::Ok(_) => Verdict::Ok(true),
                Verdict::Refused(_) => Verdict::Ok(false),
            }
        }
    }
}
