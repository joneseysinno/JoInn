//! Structural refusal constructor.

use super::{CheckId, Refusal, Subject};

impl Refusal {
    /// Structural refusal with no counter-example (contract / parse only).
    pub fn structural(check: CheckId, reason: impl Into<String>) -> Self {
        Self {
            check,
            subject: Subject::Other(String::new()),
            reason: reason.into(),
            counterexample: None,
            seed: 0,
        }
    }
}
