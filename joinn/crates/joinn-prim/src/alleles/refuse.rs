//! Shared allele refusal.

use joinn_frame::{CheckId, Refusal, Subject};

pub(crate) fn refuse(reason: &str) -> Refusal {
    Refusal {
        check: CheckId::Laws,
        subject: Subject::Allele(String::new()),
        reason: reason.into(),
        counterexample: None,
        seed: 0,
    }
}
