//! Structural refusal helper for joinn-tower.

use joinn_frame::{CheckId, Refusal, Verdict};

/// A refusal of a tower fact or request. One leaf, one function.
pub(crate) fn refuse<T>(reason: impl Into<String>) -> Verdict<T> {
    Verdict::Refused(Refusal::structural(CheckId::Other, reason.into()))
}
