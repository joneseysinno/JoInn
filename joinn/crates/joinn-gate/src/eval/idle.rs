//! Idle oracle for predicates (no `self`).

use crate::oracle::Oracle;
use joinn_frame::{CheckId, Refusal, Subject, Value, Verdict};
use std::collections::BTreeMap;

pub(in crate::eval) struct Idle;

impl Oracle for Idle {
    fn apply(&self, _inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        Verdict::Refused(Refusal {
            check: CheckId::Laws,
            subject: Subject::Other(String::new()),
            reason: "predicate has no self".into(),
            counterexample: None,
            seed: 0,
        })
    }
}
