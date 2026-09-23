//! Injected disagreement: sealed add vs difference.

use joinn_frame::Verdict;

/// Injected disagreement: sealed add vs difference. Must be a truth violation.
pub fn agree_injected_disagreement(seed: u64, n: u32) -> Verdict<String> {
    #[cfg(any(test, feature = "mutants"))]
    {
        use crate::alleles::AddInt;
        use crate::mutants::Difference;
        use joinn_frame::{Frame, IntFrame};
        use joinn_gate::Oracle;
        use std::collections::BTreeMap;

        use super::truth_violation::truth_violation;

        let int = IntFrame::new();
        for i in 0..n {
            let a = int.generate(seed.wrapping_add(u64::from(i)), 8);
            let b = int.generate(seed.wrapping_add(u64::from(i) + 3), 8);
            let inputs = BTreeMap::from([(0, a), (1, b)]);
            let se = match AddInt.apply(&inputs) {
                Verdict::Ok(m) => m,
                Verdict::Refused(r) => return Verdict::Refused(r),
            };
            let di = match Difference.apply(&inputs) {
                Verdict::Ok(m) => m,
                Verdict::Refused(r) => return Verdict::Refused(r),
            };
            if se != di {
                return Verdict::Refused(truth_violation(
                    "int.add",
                    seed.wrapping_add(u64::from(i)),
                    &inputs,
                    se.values().next().cloned(),
                    di.values().next().cloned(),
                    "reference and sealed alleles disagree",
                ));
            }
        }
        Verdict::Ok("injected disagreement was not found".into())
    }
    #[cfg(not(any(test, feature = "mutants")))]
    {
        use super::refuse::refuse;
        let _ = (seed, n);
        Verdict::Refused(refuse("injected disagreement requires the mutant register"))
    }
}

#[cfg(test)]
mod tests {
    use super::agree_injected_disagreement;
    use joinn_frame::Verdict;

    #[test]
    fn injected_disagreement_is_a_truth_violation() {
        match agree_injected_disagreement(1, 8) {
            Verdict::Refused(r) => {
                assert!(r.reason.contains("TRUTH VIOLATION"), "{}", r.reason);
                assert!(r.counterexample.is_some());
            }
            Verdict::Ok(s) => panic!("injected disagreement must not agree: {s}"),
        }
    }
}
