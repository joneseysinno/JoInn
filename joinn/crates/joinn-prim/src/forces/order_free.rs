//! Order-freedom: a combine's result does not depend on member order.

use joinn_frame::{
    CheckId, CounterExample, DEFAULT_SIZE, Frame, Refusal, SeedRng, Subject, Value, Verdict,
};
use joinn_gate::Oracle;
use std::collections::BTreeMap;
use std::num::NonZeroU32;

/// Draws `bound` pairs, then `bound` triples, from `frame`'s generator under
/// `seed`, and checks `f(a, b) = f(b, a)` on each pair and
/// `f(f(a, b), c) = f(a, f(b, c))` on each triple. `f` reads in-ports 0 and 1
/// and answers on out-port 2. The first counterexample is the refusal.
pub fn order_free(
    oracle: &dyn Oracle,
    frame: &dyn Frame,
    seed: u64,
    bound: NonZeroU32,
) -> Verdict<()> {
    let mut rng = SeedRng::new(seed);
    let refusal = |reason: String, bindings: &[(&str, &Value)], sides: Option<(&Value, &Value)>| {
        Verdict::Refused(Refusal {
            check: CheckId::Laws,
            subject: Subject::Law("order-free".into()),
            reason,
            counterexample: Some(CounterExample {
                bindings: bindings
                    .iter()
                    .map(|(name, v)| ((*name).to_owned(), (*v).clone()))
                    .collect(),
                expected: sides.map(|(x, _)| x.clone()),
                got: sides.map(|(_, y)| y.clone()),
            }),
            seed,
        })
    };
    let apply = |a: &Value, b: &Value| -> Result<Value, String> {
        match oracle.apply(&BTreeMap::from([(0, a.clone()), (1, b.clone())])) {
            Verdict::Ok(out) => out.get(&2).cloned().ok_or_else(|| {
                format!(
                    "not order-free: f({}, {}) gave no out-port 2; acceptance is a response with in-ports 0 and 1 and out-port 2",
                    frame.print(a),
                    frame.print(b)
                )
            }),
            Verdict::Refused(r) => Err(format!(
                "not order-free: the response refused f({}, {}): {}; acceptance is a response that answers every sampled pair",
                frame.print(a),
                frame.print(b),
                r.reason
            )),
        }
    };
    for _ in 0..bound.get() {
        let a = frame.generate(rng.next_u64(), DEFAULT_SIZE);
        let b = frame.generate(rng.next_u64(), DEFAULT_SIZE);
        let (ab, ba) = match (apply(&a, &b), apply(&b, &a)) {
            (Ok(ab), Ok(ba)) => (ab, ba),
            (Err(reason), _) | (_, Err(reason)) => {
                return refusal(reason, &[("a", &a), ("b", &b)], None);
            }
        };
        if !frame.eq(&ab, &ba) {
            return refusal(
                format!(
                    "not order-free: f(a, b) = {} but f(b, a) = {} at a = {}, b = {}; acceptance is a response whose result does not depend on member order",
                    frame.print(&ab),
                    frame.print(&ba),
                    frame.print(&a),
                    frame.print(&b)
                ),
                &[("a", &a), ("b", &b)],
                Some((&ab, &ba)),
            );
        }
    }
    for _ in 0..bound.get() {
        let a = frame.generate(rng.next_u64(), DEFAULT_SIZE);
        let b = frame.generate(rng.next_u64(), DEFAULT_SIZE);
        let c = frame.generate(rng.next_u64(), DEFAULT_SIZE);
        let sides = apply(&a, &b)
            .and_then(|ab| apply(&ab, &c))
            .and_then(|left| {
                apply(&b, &c)
                    .and_then(|bc| apply(&a, &bc))
                    .map(|right| (left, right))
            });
        let (left, right) = match sides {
            Ok(pair) => pair,
            Err(reason) => return refusal(reason, &[("a", &a), ("b", &b), ("c", &c)], None),
        };
        if !frame.eq(&left, &right) {
            return refusal(
                format!(
                    "not order-free: f(f(a, b), c) = {} but f(a, f(b, c)) = {} at a = {}, b = {}, c = {}; acceptance is a response whose result does not depend on member order",
                    frame.print(&left),
                    frame.print(&right),
                    frame.print(&a),
                    frame.print(&b),
                    frame.print(&c)
                ),
                &[("a", &a), ("b", &b), ("c", &c)],
                Some((&left, &right)),
            );
        }
    }
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use super::order_free;
    use crate::forces::{REGISTER_BOUND, REGISTER_SEED};
    use crate::natives_with_mutants; // allow(vocab): tests inject mutants
    use joinn_dna::NativeId;
    use joinn_frame::{IntFrame, Verdict};

    fn run(native: &str) -> Verdict<()> {
        let natives = natives_with_mutants(); // allow(vocab): tests inject mutants
        let Some(oracle) = natives.get(&NativeId(native.into())) else {
            panic!("{native} is not registered");
        };
        order_free(oracle, &IntFrame::new(), REGISTER_SEED, REGISTER_BOUND)
    }

    #[test]
    fn the_sum_allele_is_order_free() {
        assert!(matches!(run("add@ℤ"), Verdict::Ok(())));
    }

    #[test]
    fn difference_is_refused_on_commutativity() {
        match run("mutant.difference") {
            Verdict::Refused(r) => {
                println!("{}", r.reason);
                assert!(
                    r.reason.starts_with("not order-free: f(a, b) = "),
                    "{}",
                    r.reason
                );
                assert!(r.reason.contains(" but f(b, a) = "), "{}", r.reason);
            }
            Verdict::Ok(()) => panic!("difference passed order-free"),
        }
    }

    #[test]
    fn midpoint_is_refused_on_associativity() {
        match run("mutant.midpoint") {
            Verdict::Refused(r) => {
                println!("{}", r.reason);
                assert!(
                    r.reason.starts_with("not order-free: f(f(a, b), c) = "),
                    "{}",
                    r.reason
                );
                assert!(r.reason.contains(" but f(a, f(b, c)) = "), "{}", r.reason);
            }
            Verdict::Ok(()) => panic!("midpoint passed order-free"),
        }
    }

    #[test]
    fn max_and_plus1_are_order_free_though_wrong() {
        assert!(matches!(run("mutant.max"), Verdict::Ok(())));
        assert!(matches!(run("mutant.plus1"), Verdict::Ok(())));
    }
}
