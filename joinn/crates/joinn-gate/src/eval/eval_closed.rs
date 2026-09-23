//! `eval_closed`.

use crate::budget::Budget;
use crate::oracle::Oracle;
use joinn_dna::Formula;
use joinn_frame::{CheckId, FrameRef, FrameRegistry, Refusal, Subject, Verdict};
use std::collections::BTreeMap;

use super::CellOracles;

pub(in crate::eval) fn eval_closed(
    formula: &Formula,
    frames: &FrameRegistry,
    oracle: &dyn Oracle,
    cells: &CellOracles,
    allele_frame: FrameRef,
    budget: &Budget,
) -> Verdict<()> {
    match super::eval_formula::eval_formula(
        formula,
        &BTreeMap::new(),
        frames,
        oracle,
        cells,
        allele_frame,
        budget.seed,
    ) {
        Verdict::Ok(true) => Verdict::Ok(()),
        Verdict::Ok(false) => Verdict::Refused(Refusal {
            check: CheckId::Laws,
            subject: Subject::Law(String::new()),
            reason: "closed formula is false".into(),
            counterexample: None,
            seed: budget.seed,
        }),
        Verdict::Refused(r) => Verdict::Refused(r),
    }
}
