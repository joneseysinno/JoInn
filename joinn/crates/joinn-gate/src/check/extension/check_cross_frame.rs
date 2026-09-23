//! Check 4: conservative extension and the six cross-frame obligations of §2.5.

use crate::budget::Budget;
use crate::oracle::{Catching, Oracle};
use joinn_dna::{Cell, Direction};
use joinn_frame::{CheckId, CounterExample, FrameRef, FrameRegistry, Refusal, Subject, Verdict};
use std::collections::BTreeMap;

/// Cross-frame admission of an allele whose frame is wider than the coding frame.
pub fn check_cross_frame(
    cell: &Cell,
    allele_frame: FrameRef,
    frames: &FrameRegistry,
    oracle: &dyn Oracle,
    rust_int: Option<&dyn Oracle>,
    budget: &Budget,
) -> Verdict<()> {
    if allele_frame == cell.coding.frame {
        return Verdict::Ok(());
    }
    let Some(wide) = frames.get(&allele_frame) else {
        return Verdict::Refused(Refusal::structural(
            CheckId::Extension,
            format!("unregistered allele frame {allele_frame}"),
        ));
    };
    if !wide.extensions().contains(&cell.coding.frame) {
        return Verdict::Refused(Refusal {
            check: CheckId::Extension,
            subject: Subject::Frame(allele_frame.to_string()),
            reason: format!(
                "{allele_frame} does not declare an embedding of {}",
                cell.coding.frame
            ),
            counterexample: None,
            seed: budget.seed,
        });
    }
    // ρ ∘ ι = id on sampled generators + founding values.
    let Some(narrow) = frames.get(&cell.coding.frame) else {
        return Verdict::Refused(Refusal::structural(
            CheckId::Extension,
            "coding frame missing",
        ));
    };
    for i in 0..budget.law_samples {
        let x = narrow.generate(budget.seed.wrapping_add(u64::from(i)), budget.size);
        match (wide.embed(&cell.coding.frame, &x), ()) {
            (Verdict::Ok(ix), ()) => match wide.restrict(&cell.coding.frame, &ix) {
                Verdict::Ok(back) if narrow.eq(&back, &x) => {}
                _ => {
                    return Verdict::Refused(Refusal {
                        check: CheckId::Extension,
                        subject: Subject::Frame(allele_frame.to_string()),
                        reason: "ρ ∘ ι ≠ id on the coding frame".into(),
                        counterexample: Some(CounterExample {
                            bindings: BTreeMap::from([("x".into(), x)]),
                            expected: None,
                            got: None,
                        }),
                        seed: budget.seed,
                    });
                }
            },
            (Verdict::Refused(r), ()) => return Verdict::Refused(r),
        }
    }
    // Restricted allele agrees with the ℤ allele / founding witnesses.
    if let Some(int_oracle) = rust_int {
        let in_ports: Vec<u32> = cell
            .coding
            .contract
            .ports
            .iter()
            .filter(|p| p.direction == Direction::In)
            .map(|p| p.position)
            .collect();
        let out = cell
            .coding
            .contract
            .ports
            .iter()
            .find(|p| p.direction == Direction::Out)
            .map(|p| p.position)
            .unwrap_or(2);
        for i in 0..budget.law_samples {
            let mut inputs_n = BTreeMap::new();
            let mut inputs_w = BTreeMap::new();
            for pos in &in_ports {
                let x = narrow.generate(
                    budget
                        .seed
                        .wrapping_add(u64::from(i).wrapping_mul(17) + u64::from(*pos)),
                    budget.size,
                );
                let ix = match wide.embed(&cell.coding.frame, &x) {
                    Verdict::Ok(v) => v,
                    Verdict::Refused(r) => return Verdict::Refused(r),
                };
                inputs_n.insert(*pos, x);
                inputs_w.insert(*pos, ix);
            }
            let got_n = match (Catching { inner: int_oracle }).apply(&inputs_n) {
                Verdict::Ok(o) => o,
                Verdict::Refused(r) => return Verdict::Refused(r),
            };
            let got_w = match (Catching { inner: oracle }).apply(&inputs_w) {
                Verdict::Ok(o) => o,
                Verdict::Refused(r) => return Verdict::Refused(r),
            };
            let Some(wn) = got_n.get(&out) else {
                continue;
            };
            let Some(ww) = got_w.get(&out) else {
                return Verdict::Refused(Refusal {
                    check: CheckId::Extension,
                    subject: Subject::Allele(String::new()),
                    reason: "wider allele produced no out-port".into(),
                    counterexample: Some(CounterExample {
                        bindings: super::named::named(&inputs_n),
                        expected: Some(wn.clone()),
                        got: None,
                    }),
                    seed: budget.seed,
                });
            };
            match wide.restrict(&cell.coding.frame, ww) {
                Verdict::Ok(back) if narrow.eq(&back, wn) => {}
                Verdict::Ok(back) => {
                    return Verdict::Refused(Refusal {
                        check: CheckId::Extension,
                        subject: Subject::Allele(String::new()),
                        reason: "restricted allele disagrees with the coding-frame allele".into(),
                        counterexample: Some(CounterExample {
                            bindings: super::named::named(&inputs_n),
                            expected: Some(wn.clone()),
                            got: Some(back),
                        }),
                        seed: budget.seed,
                    });
                }
                Verdict::Refused(_) => {
                    return Verdict::Refused(Refusal {
                        check: CheckId::Extension,
                        subject: Subject::Allele(String::new()),
                        reason: "wider allele does not restrict to the coding frame".into(),
                        counterexample: Some(CounterExample {
                            bindings: super::named::named(&inputs_n),
                            expected: Some(wn.clone()),
                            got: Some(ww.clone()),
                        }),
                        seed: budget.seed,
                    });
                }
            }
        }
    }
    Verdict::Ok(())
}
