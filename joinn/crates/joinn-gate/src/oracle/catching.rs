//! Panic-catching oracle wrapper.

use super::Oracle;
use joinn_frame::{CheckId, Refusal, Subject, Value, Verdict};
use std::collections::BTreeMap;
use std::panic::{AssertUnwindSafe, catch_unwind};

/// Catch panics at the allele boundary (mutant 9).
pub struct Catching<'a> {
    /// Inner oracle.
    pub inner: &'a dyn Oracle,
}

impl Oracle for Catching<'_> {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let result = catch_unwind(AssertUnwindSafe(|| self.inner.apply(inputs)));
        std::panic::set_hook(prev);
        match result {
            Ok(v) => v,
            Err(_) => Verdict::Refused(Refusal {
                check: CheckId::Laws,
                subject: Subject::Allele(String::new()),
                reason: "allele panicked".into(),
                counterexample: None,
                seed: 0,
            }),
        }
    }
}
