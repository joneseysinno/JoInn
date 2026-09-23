//! `Gate::admit_cell`.

use super::{Accepted, Gate};
use crate::check;
use joinn_dna::Cell;
use joinn_frame::Verdict;

pub(in crate::gate) fn admit_cell(
    gate: &Gate,
    proposed: &Cell,
    parent: Option<&Cell>,
) -> Verdict<Accepted> {
    match check::contract::check(proposed, &gate.frames) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Verdict::Refused(r),
    }
    if let Some(old) = parent {
        match check::extension::check_parent(proposed, old, &gate.budget) {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
    }
    match super::check_turns::check_turns(gate, proposed) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Verdict::Refused(r),
    }
    Verdict::Ok(Accepted {
        advisory: check::laws::degeneracy_advisory(proposed),
    })
}

#[cfg(test)]
mod tests {
    use super::super::Gate;
    use crate::budget::Budget;
    use joinn_dna::{TurnDecl, sum_cell};
    use joinn_frame::Verdict;

    #[test]
    fn turn_without_allele_is_refused() {
        let mut cell = sum_cell();
        cell.coding.turns = vec![TurnDecl {
            out: 0,
            from: vec![1, 2],
        }];
        let gate = Gate::phase1(Budget::default());
        match gate.admit_cell(&cell, None) {
            Verdict::Refused(r) => {
                assert!(
                    r.reason.contains("turn") || r.reason.contains("allele"),
                    "{}",
                    r.reason
                );
            }
            Verdict::Ok(_) => panic!("unwitnessed turn must be refused by admit_cell"),
        }
    }
}
