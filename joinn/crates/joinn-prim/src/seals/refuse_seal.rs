//! Refusal attributed to a sealed native.

use joinn_frame::{CheckId, Refusal, Subject};

pub(in crate::seals) fn refuse_seal(sealed: &str, reason: &str) -> Refusal {
    Refusal {
        check: CheckId::Laws,
        subject: Subject::Allele(sealed.into()),
        reason: reason.into(),
        counterexample: None,
        seed: 0,
    }
}
