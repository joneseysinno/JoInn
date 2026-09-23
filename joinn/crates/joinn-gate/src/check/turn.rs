//! Check 5: a declared turn has a registered allele that round-trips.

mod sum_ports;

pub use sum_ports::sum_ports;

use crate::oracle::Oracle;
use joinn_dna::{Cell, Direction, TurnDecl};
use joinn_frame::{CheckId, CounterExample, Frame, Refusal, Subject, Verdict};
use std::collections::BTreeMap;

fn refuse(reason: &str) -> Refusal {
    Refusal {
        check: CheckId::Laws,
        subject: Subject::Allele("turn".into()),
        reason: reason.into(),
        counterexample: None,
        seed: 0,
    }
}

/// Generated round-trip: `self@o(k: turn@k(S), …I) = the value drawn at o`.
pub fn check(
    turn: &TurnDecl,
    forward: &dyn Oracle,
    turn_allele: &dyn Oracle,
    seed: u64,
    n: u32,
    ins: &[u32],
    fwd_out: u32,
    frame: &dyn Frame,
) -> Verdict<()> {
    if turn.from.is_empty() {
        return Verdict::Refused(refuse("turn from-set is empty"));
    }
    for i in 0..n {
        let mut ins_v = BTreeMap::new();
        for p in ins {
            ins_v.insert(
                *p,
                frame.generate(
                    seed.wrapping_add(u64::from(i).wrapping_mul(17) + u64::from(*p)),
                    8,
                ),
            );
        }
        let fwd = match forward.apply(&ins_v) {
            Verdict::Ok(m) => m,
            Verdict::Refused(r) => {
                return Verdict::Refused(Refusal {
                    check: CheckId::Laws,
                    subject: Subject::Allele("turn".into()),
                    reason: format!("TRUTH VIOLATION turn: forward refused ({})", r.reason),
                    counterexample: None,
                    seed: seed.wrapping_add(u64::from(i)),
                });
            }
        };
        let Some(s) = fwd.get(&fwd_out) else {
            return Verdict::Refused(refuse("forward produced no out"));
        };
        let mut assignment = ins_v;
        assignment.insert(fwd_out, s.clone());
        let mut t_in = BTreeMap::new();
        for p in &turn.from {
            let Some(v) = assignment.get(p) else {
                return Verdict::Refused(refuse(&format!(
                    "turn from-port {p} is not in the assignment"
                )));
            };
            t_in.insert(*p, v.clone());
        }
        let turned = match turn_allele.apply(&t_in) {
            Verdict::Ok(m) => m,
            Verdict::Refused(r) => {
                return Verdict::Refused(Refusal {
                    check: CheckId::Laws,
                    subject: Subject::Allele("turn".into()),
                    reason: format!(
                        "TRUTH VIOLATION turn: turn allele refused ({}) (seed {})",
                        r.reason,
                        seed.wrapping_add(u64::from(i))
                    ),
                    counterexample: r.counterexample,
                    seed: seed.wrapping_add(u64::from(i)),
                });
            }
        };
        let expected = assignment.get(&turn.out);
        let got = turned.get(&turn.out);
        if got != expected {
            let mut bindings = BTreeMap::new();
            for (k, v) in &assignment {
                bindings.insert(format!("p{k}"), v.clone());
            }
            return Verdict::Refused(Refusal {
                check: CheckId::Laws,
                subject: Subject::Allele("turn".into()),
                reason: format!(
                    "TRUTH VIOLATION turn: reference and turn alleles disagree (seed {})",
                    seed.wrapping_add(u64::from(i))
                ),
                counterexample: Some(CounterExample {
                    bindings,
                    expected: expected.cloned(),
                    got: got.cloned(),
                }),
                seed: seed.wrapping_add(u64::from(i)),
            });
        }
    }
    Verdict::Ok(())
}

/// Names under which a turn allele may be registered. Derived, not special-cased.
pub fn native_names(forward: &str, k: u32) -> Vec<String> {
    vec![format!("{forward}.turn{k}")]
}

/// In-ports and out-port from a cell's contract.
pub fn ports_from_contract(cell: &Cell) -> (Vec<u32>, u32) {
    let mut ins = Vec::new();
    let mut out = 0u32;
    for p in &cell.coding.contract.ports {
        match p.direction {
            Direction::In => ins.push(p.position),
            Direction::Out => out = p.position,
        }
    }
    if ins.is_empty() {
        ins.extend_from_slice(&[0, 1]);
        out = 2;
    }
    (ins, out)
}
