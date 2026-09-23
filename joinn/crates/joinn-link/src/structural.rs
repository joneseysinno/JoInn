//! Structural refusal helper for link checks.

use joinn_frame::{CheckId, Refusal, Verdict};

/// A structural refusal. One leaf, one function.
pub(crate) fn refuse<T>(reason: impl Into<String>) -> Verdict<T> {
    Verdict::Refused(Refusal::structural(CheckId::Other, reason.into()))
}
