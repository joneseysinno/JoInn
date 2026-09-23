//! Check every stated opposition on samples.

use joinn_frame::{CheckId, Refusal, Subject, Verdict};

use super::check_one::check_one;
use super::check_pairing::check_pairing;
use super::evaluable::evaluable;

/// Check every stated opposition on `n` samples from `seed`.
pub fn check_oppositions(seed: u64, n: u32) -> Verdict<()> {
    match check_pairing() {
        Verdict::Ok(_) => {}
        Verdict::Refused(r) => return Verdict::Refused(r),
    }
    for p in evaluable() {
        if let Some(reason) = check_one(p.as_ref(), seed, n) {
            return Verdict::Refused(Refusal {
                check: CheckId::Laws,
                subject: Subject::Allele(p.name().into()),
                reason,
                counterexample: None,
                seed,
            });
        }
    }
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::check_pairing::check_pairing;
    use super::super::register::register;
    use super::check_oppositions;
    use joinn_frame::Verdict;

    #[test]
    fn floor_oppositions_hold_on_10000_samples() {
        match check_oppositions(23, 10_000) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
        match check_pairing() {
            Verdict::Ok(pairs) => {
                assert!(!pairs.is_empty());
                assert!(register().len().is_multiple_of(2));
            }
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }
}
