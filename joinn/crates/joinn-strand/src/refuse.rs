//! Structural refusal helper for joinn-strand.

use joinn_frame::{CheckId, Refusal, Verdict};

/// A parse refusal. One leaf, one function.
pub(crate) fn refuse<T>(reason: impl Into<String>) -> Verdict<T> {
    Verdict::Refused(Refusal::structural(CheckId::Parse, reason.into()))
}
