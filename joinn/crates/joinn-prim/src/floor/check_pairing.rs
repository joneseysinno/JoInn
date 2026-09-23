//! Pairing: every member's opposite is in the floor and names it back.

use joinn_frame::{CheckId, Refusal, Subject, Verdict};
use std::collections::BTreeMap;

use super::Opposition;
use super::register::register;

/// Pairing: every member's opposite is in the floor and names it back; the count is even.
pub fn check_pairing() -> Verdict<Vec<(String, String)>> {
    let floor = register();
    let names: BTreeMap<String, Opposition> = floor
        .iter()
        .map(|p| (p.name().to_string(), p.opposition()))
        .collect();
    let mut pairs = Vec::new();
    let mut seen = BTreeMap::new();
    for p in &floor {
        match p.opposition() {
            Opposition::Undeclared => {
                return Verdict::Refused(Refusal {
                    check: CheckId::Laws,
                    subject: Subject::Allele(p.name().into()),
                    reason: format!("{} has no opposition", p.name()),
                    counterexample: None,
                    seed: 0,
                });
            }
            Opposition::Inverse(o) => {
                if !names.contains_key(o) {
                    return Verdict::Refused(Refusal {
                        check: CheckId::Laws,
                        subject: Subject::Allele(p.name().into()),
                        reason: format!(
                            "{} names opposite {o}, which is not in the floor",
                            p.name()
                        ),
                        counterexample: None,
                        seed: 0,
                    });
                }
                match names.get(o) {
                    Some(Opposition::Inverse(back)) if *back == p.name() => {}
                    Some(Opposition::Inverse(back)) => {
                        return Verdict::Refused(Refusal {
                            check: CheckId::Laws,
                            subject: Subject::Allele(p.name().into()),
                            reason: format!(
                                "{o} names {back}, not {}; the pair is not a pair",
                                p.name()
                            ),
                            counterexample: None,
                            seed: 0,
                        });
                    }
                    _ => {
                        return Verdict::Refused(Refusal {
                            check: CheckId::Laws,
                            subject: Subject::Allele(o.into()),
                            reason: format!("{o} does not name {} back", p.name()),
                            counterexample: None,
                            seed: 0,
                        });
                    }
                }
                let a = p.name();
                let (lo, hi) = if a < o { (a, o) } else { (o, a) };
                if seen.insert((lo.to_string(), hi.to_string()), ()).is_none() {
                    pairs.push((lo.to_string(), hi.to_string()));
                }
            }
        }
    }
    if !floor.len().is_multiple_of(2) {
        return Verdict::Refused(Refusal {
            check: CheckId::Laws,
            subject: Subject::Other("floor".into()),
            reason: format!(
                "floor count {} is odd; an opposition is missing",
                floor.len()
            ),
            counterexample: None,
            seed: 0,
        });
    }
    pairs.sort();
    Verdict::Ok(pairs)
}

#[cfg(test)]
mod tests {
    use super::super::{Opposition, Reference, Register};
    use joinn_frame::{Value, Verdict};
    use joinn_gate::Oracle;
    use std::collections::BTreeMap;

    struct UndeclaredPrim;
    impl Oracle for UndeclaredPrim {
        fn apply(&self, _inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
            Verdict::Ok(BTreeMap::new())
        }
    }
    impl Reference for UndeclaredPrim {
        fn name(&self) -> &'static str {
            "undeclared"
        }
        fn opposition(&self) -> Opposition {
            Opposition::Undeclared
        }
        fn register(&self) -> Register {
            Register::Matter
        }
    }

    #[test]
    fn undeclared_opposition_is_refused() {
        let p = UndeclaredPrim;
        assert_eq!(p.opposition(), Opposition::Undeclared);
        let reason = format!("{} has no opposition", p.name());
        assert!(reason.contains("undeclared"));
    }
}
