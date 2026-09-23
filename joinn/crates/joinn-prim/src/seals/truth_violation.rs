//! Build a truth-violation refusal.

use joinn_frame::{CheckId, CounterExample, Refusal, Subject, Value};
use std::collections::BTreeMap;

pub(in crate::seals) fn truth_violation(
    name: &str,
    seed: u64,
    inputs: &BTreeMap<u32, Value>,
    expected: Option<Value>,
    got: Option<Value>,
    msg: &str,
) -> Refusal {
    let mut bindings = BTreeMap::new();
    for (k, v) in inputs {
        bindings.insert(format!("in{k}"), v.clone());
    }
    Refusal {
        check: CheckId::Laws,
        subject: Subject::Allele(name.into()),
        reason: format!("TRUTH VIOLATION {name}: {msg} (seed {seed})"),
        counterexample: Some(CounterExample {
            bindings,
            expected,
            got,
        }),
        seed,
    }
}
