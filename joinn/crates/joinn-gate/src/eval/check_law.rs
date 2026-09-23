//! `check_law`.

use crate::budget::Budget;
use crate::oracle::Oracle;
use joinn_dna::{Formula, Law};
use joinn_frame::{CheckId, FrameRef, FrameRegistry, Refusal, Subject, Verdict};

use super::{CellOracles, LawReport};

pub fn check_law(
    law: &Law,
    frames: &FrameRegistry,
    oracle: &dyn Oracle,
    cells: &CellOracles,
    allele_frame: FrameRef,
    budget: &Budget,
) -> Verdict<LawReport> {
    if budget.law_samples == 0 {
        return Verdict::Refused(Refusal {
            check: CheckId::Laws,
            subject: Subject::Law(law.name.0.clone()),
            reason: "law produced 0 samples; an uncheckable law must not pass".into(),
            counterexample: None,
            seed: budget.seed,
        });
    }
    match &law.formula {
        Formula::ForAll { vars, body } => super::eval_forall::eval_forall(
            &law.name.0,
            vars,
            body,
            frames,
            oracle,
            cells,
            allele_frame,
            budget,
        ),
        other => {
            super::eval_closed::eval_closed(other, frames, oracle, cells, allele_frame, budget).map(
                |_| LawReport {
                    name: law.name.0.clone(),
                    samples: 1,
                },
            )
        }
    }
}
