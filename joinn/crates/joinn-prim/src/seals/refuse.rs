//! Shared seal refusal.

use joinn_frame::{CheckId, Refusal, Subject};

pub(in crate::seals) fn refuse(reason: &str) -> Refusal {
    Refusal {
        check: CheckId::Laws,
        subject: Subject::Allele(String::new()),
        reason: reason.into(),
        counterexample: None,
        seed: 0,
    }
}
