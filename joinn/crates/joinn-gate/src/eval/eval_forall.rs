//! `eval_forall`.

use crate::budget::Budget;
use crate::oracle::Oracle;
use joinn_dna::{Formula, VarId};
use joinn_frame::{
    CheckId, CounterExample, FrameRef, FrameRegistry, Refusal, SeedRng, Subject, Verdict,
};
use std::collections::BTreeMap;

use super::{CellOracles, LawReport};

pub(in crate::eval) fn eval_forall(
    name: &str,
    vars: &[(VarId, FrameRef)],
    body: &Formula,
    frames: &FrameRegistry,
    oracle: &dyn Oracle,
    cells: &CellOracles,
    allele_frame: FrameRef,
    budget: &Budget,
) -> Verdict<LawReport> {
    let mut rng = SeedRng::new(budget.seed);
    // The calculator's named hole: parse("007") = 7 must not sneak past
    // independently sampled Text and ℤ binders.
    if vars.iter().any(|(_, f)| f.id == joinn_frame::FrameId::Text)
        && vars.iter().any(|(_, f)| f.id == joinn_frame::FrameId::Int)
    {
        let text = joinn_frame::TextFrame::new();
        let int = joinn_frame::IntFrame::new();
        let hole = match (
            joinn_frame::Frame::canonicalize(&text, joinn_frame::Term::text("007")),
            joinn_frame::Frame::canonicalize(&int, joinn_frame::Term::int(7)),
        ) {
            (Verdict::Ok(s), Verdict::Ok(n)) => Some((s, n)),
            _ => None,
        };
        if let Some((s_val, n_val)) = hole {
            let mut env = BTreeMap::new();
            for (var, fr) in vars {
                env.insert(
                    var.0.clone(),
                    if fr.id == joinn_frame::FrameId::Text {
                        s_val.clone()
                    } else if fr.id == joinn_frame::FrameId::Int {
                        n_val.clone()
                    } else {
                        frames
                            .get(fr)
                            .map(|f| f.generate(budget.seed, budget.size))
                            .unwrap_or_else(|| n_val.clone())
                    },
                );
            }
            match super::eval_formula::eval_formula(
                body,
                &env,
                frames,
                oracle,
                cells,
                allele_frame,
                budget.seed,
            ) {
                Verdict::Ok(true) => {}
                Verdict::Ok(false) => {
                    return Verdict::Refused(Refusal {
                        check: CheckId::Laws,
                        subject: Subject::Law(name.into()),
                        reason: format!("law {name} does not hold on the 007 hole"),
                        counterexample: Some(CounterExample {
                            bindings: env,
                            expected: None,
                            got: None,
                        }),
                        seed: budget.seed,
                    });
                }
                Verdict::Refused(r) => {
                    return Verdict::Refused(Refusal {
                        check: CheckId::Laws,
                        subject: Subject::Law(name.into()),
                        reason: format!("law {name}: {}", r.reason),
                        counterexample: r.counterexample,
                        seed: budget.seed,
                    });
                }
            }
        }
    }
    for i in 0..budget.law_samples {
        let mut env = BTreeMap::new();
        for (var, fr) in vars {
            let sample_frame = if allele_frame != *fr && allele_frame == FrameRef::rat() {
                allele_frame
            } else {
                *fr
            };
            let Some(frame) = frames.get(&sample_frame) else {
                return Verdict::Refused(Refusal {
                    check: CheckId::Laws,
                    subject: Subject::Law(name.into()),
                    reason: format!("unregistered frame {sample_frame}"),
                    counterexample: None,
                    seed: budget.seed,
                });
            };
            let seed = rng.next_u64();
            env.insert(var.0.clone(), frame.generate(seed, budget.size));
        }
        match super::eval_formula::eval_formula(
            body,
            &env,
            frames,
            oracle,
            cells,
            allele_frame,
            budget.seed,
        ) {
            Verdict::Ok(true) => {}
            Verdict::Ok(false) => {
                let shrunk = super::shrink_env::shrink_env(
                    vars,
                    body,
                    env,
                    frames,
                    oracle,
                    cells,
                    allele_frame,
                    budget,
                );
                return Verdict::Refused(Refusal {
                    check: CheckId::Laws,
                    subject: Subject::Law(name.into()),
                    reason: format!("law {name} does not hold on a sampled tuple"),
                    counterexample: Some(CounterExample {
                        bindings: shrunk,
                        expected: None,
                        got: None,
                    }),
                    seed: budget.seed.wrapping_add(u64::from(i)),
                });
            }
            Verdict::Refused(r) => {
                return Verdict::Refused(Refusal {
                    check: CheckId::Laws,
                    subject: Subject::Law(name.into()),
                    reason: format!("law {name}: {}", r.reason),
                    counterexample: r.counterexample,
                    seed: budget.seed,
                });
            }
        }
    }
    Verdict::Ok(LawReport {
        name: name.into(),
        samples: budget.law_samples,
    })
}
