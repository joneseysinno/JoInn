//! Check 4: conservative extension — parent/subtype sampling.

use crate::budget::Budget;
use joinn_dna::{Cell, Direction};
use joinn_frame::{CheckId, Refusal, Subject, Verdict};

pub fn check_parent(proposed: &Cell, parent: &Cell, budget: &Budget) -> Verdict<String> {
    let old_in: Vec<u32> = parent
        .coding
        .contract
        .ports
        .iter()
        .filter(|p| p.direction == Direction::In)
        .map(|p| p.position)
        .collect();
    let new_in: Vec<u32> = proposed
        .coding
        .contract
        .ports
        .iter()
        .filter(|p| p.direction == Direction::In)
        .map(|p| p.position)
        .collect();
    for pos in &new_in {
        if !old_in.contains(pos) {
            let declared = proposed.coding.lineage.is_some();
            if !declared {
                return Verdict::Refused(Refusal {
                    check: CheckId::Extension,
                    subject: Subject::Other(String::new()),
                    reason: format!(
                        "added in-port {pos} is a non-conservative extension; declare a new version (lineage)"
                    ),
                    counterexample: None,
                    seed: budget.seed,
                });
            }
        }
    }
    Verdict::Ok(format!(
        "not refuted at this budget (seed {}, samples {})",
        budget.seed, budget.law_samples
    ))
}
