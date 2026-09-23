//! `Gate::admit_allele`.

use super::{Accepted, Gate};
use crate::check;
use joinn_dna::{Allele, AlleleBody, Cell, hash};
use joinn_frame::{CheckId, Refusal, Subject, Verdict};

pub(in crate::gate) fn admit_allele(
    gate: &Gate,
    cell: &Cell,
    allele: &Allele,
) -> Verdict<Accepted> {
    match check::contract::check(cell, &gate.frames) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Verdict::Refused(r),
    }
    let id = match &allele.body {
        AlleleBody::Native(id) => id,
        AlleleBody::Dna(h) => {
            return Verdict::Refused(Refusal {
                check: CheckId::Contract,
                subject: Subject::Allele(h.to_hex()),
                reason: "Dna allele admission is judged by agree, not by this path".into(),
                counterexample: None,
                seed: gate.budget.seed,
            });
        }
    };
    let Some(oracle) = gate.natives.get(id) else {
        return Verdict::Refused(Refusal {
            check: CheckId::Contract,
            subject: Subject::Allele(id.0.clone()),
            reason: format!("unknown native allele {}", id.0),
            counterexample: None,
            seed: gate.budget.seed,
        });
    };
    match check::laws::check(
        cell,
        &gate.frames,
        oracle,
        &gate.cells,
        allele.frame,
        &gate.budget,
    ) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Verdict::Refused(r),
    }
    match check::witnesses::check(cell, &gate.frames, oracle, allele.frame, gate.budget.seed) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Verdict::Refused(r),
    }
    let rust_int = if allele.frame != cell.coding.frame {
        gate.natives.get(&joinn_dna::NativeId("add@ℤ".into()))
    } else {
        None
    };
    match check::extension::check_cross_frame(
        cell,
        allele.frame,
        &gate.frames,
        oracle,
        rust_int,
        &gate.budget,
    ) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Verdict::Refused(r),
    }
    // Out-frame of outputs must match the contract (mutant 8).
    if let Some(out) = cell
        .coding
        .contract
        .ports
        .iter()
        .find(|p| p.direction == joinn_dna::Direction::Out)
    {
        if let Some(w) = cell.coding.founding.first() {
            if let Some(exp) = w.outputs.values().next() {
                if allele.frame == cell.coding.frame && exp.frame() != &out.frame {
                    return Verdict::Refused(Refusal::structural(
                        CheckId::Contract,
                        "witness out-frame does not match the port",
                    ));
                }
            }
        }
        // Probe the oracle with founding inputs if any.
        if let Some(w) = cell.coding.founding.first() {
            if let joinn_frame::Verdict::Ok(outs) = oracle.apply(&w.inputs) {
                if let Some(got) = outs.get(&out.position) {
                    if got.frame() != &out.frame && allele.frame == cell.coding.frame {
                        return Verdict::Refused(Refusal {
                            check: CheckId::Contract,
                            subject: Subject::Allele(id.0.clone()),
                            reason: "allele out-frame does not match the contract".into(),
                            counterexample: None,
                            seed: gate.budget.seed,
                        });
                    }
                }
            }
        }
    }
    let _ = hash(&cell.coding);
    Verdict::Ok(Accepted {
        advisory: check::laws::degeneracy_advisory(cell),
    })
}
