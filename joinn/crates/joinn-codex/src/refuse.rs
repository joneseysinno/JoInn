//! Structural refusal helper for joinn-codex.

use joinn_frame::{CheckId, Refusal, Verdict};

/// A refusal of the table or the reader. One leaf, one function.
pub(crate) fn refuse<T>(reason: impl Into<String>) -> Verdict<T> {
    Verdict::Refused(Refusal::structural(CheckId::Other, reason.into()))
}
