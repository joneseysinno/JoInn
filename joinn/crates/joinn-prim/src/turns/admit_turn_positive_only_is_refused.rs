//! Mutant 19: turn right on positives, wrong on negatives.

use crate::alleles::AddInt;
use crate::mutants::TurnPosOnly;
use joinn_dna::TurnDecl;
use joinn_frame::Verdict;

use super::admit_turn::admit_turn;

/// Mutant 19: a turn that is right on positives and wrong on negatives.
#[cfg(any(test, feature = "mutants"))]
pub fn admit_turn_positive_only_is_refused() -> bool {
    let t = TurnDecl {
        out: 0,
        from: vec![1, 2],
    };
    matches!(
        admit_turn(&t, &AddInt, &TurnPosOnly, 1, 64),
        Verdict::Refused(r)
            if r.reason.contains("TRUTH VIOLATION") && r.counterexample.is_some()
    )
}

#[cfg(test)]
mod tests {
    use super::admit_turn_positive_only_is_refused;

    #[test]
    fn disagreeing_turn_is_a_truth_violation() {
        assert!(admit_turn_positive_only_is_refused());
    }
}
