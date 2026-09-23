//! `Gate::check_turns`.

use super::Gate;
use crate::check;
use joinn_dna::{AlleleBody, Cell};
use joinn_frame::{CheckId, Refusal, Subject, Verdict};

pub(in crate::gate) fn check_turns(gate: &Gate, proposed: &Cell) -> Verdict<()> {
    if proposed.coding.turns.is_empty() {
        return Verdict::Ok(());
    }
    let forward_id = match proposed.alleles.first() {
        Some(a) => match &a.body {
            AlleleBody::Native(id) => id.clone(),
            AlleleBody::Dna(h) => {
                return Verdict::Refused(Refusal {
                    check: CheckId::Contract,
                    subject: Subject::Allele(h.to_hex()),
                    reason: "turn admission needs a registered native forward allele".into(),
                    counterexample: None,
                    seed: gate.budget.seed,
                });
            }
        },
        None => {
            return Verdict::Refused(Refusal {
                check: CheckId::Contract,
                subject: Subject::Other("turn".into()),
                reason: "cell with a turn block and no registered turn allele".into(),
                counterexample: None,
                seed: gate.budget.seed,
            });
        }
    };
    let Some(forward) = gate.natives.get(&forward_id) else {
        return Verdict::Refused(Refusal {
            check: CheckId::Contract,
            subject: Subject::Allele(forward_id.0.clone()),
            reason: format!("unknown forward allele {} for turn check", forward_id.0),
            counterexample: None,
            seed: gate.budget.seed,
        });
    };
    let (ins, fwd_out) = check::turn::ports_from_contract(proposed);
    let Some(frame) = gate.frames.get(&proposed.coding.frame) else {
        return Verdict::Refused(Refusal {
            check: CheckId::Contract,
            subject: Subject::Frame(proposed.coding.frame.to_string()),
            reason: format!("turn check: unknown frame {}", proposed.coding.frame),
            counterexample: None,
            seed: gate.budget.seed,
        });
    };
    for t in &proposed.coding.turns {
        let mut found = None;
        for name in check::turn::native_names(&forward_id.0, t.out) {
            if let Some(oracle) = gate.natives.get(&joinn_dna::NativeId(name)) {
                found = Some(oracle);
                break;
            }
        }
        let Some(turn_allele) = found else {
            return Verdict::Refused(Refusal {
                check: CheckId::Contract,
                subject: Subject::Allele(format!("{}.turn{}", forward_id.0, t.out)),
                reason: format!(
                    "cell with a turn block and no registered turn allele (turn {} from {:?})",
                    t.out, t.from
                ),
                counterexample: None,
                seed: gate.budget.seed,
            });
        };
        match check::turn::check(
            t,
            forward,
            turn_allele,
            gate.budget.seed,
            gate.budget.law_samples.max(8),
            &ins,
            fwd_out,
            frame,
        ) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
    }
    Verdict::Ok(())
}
