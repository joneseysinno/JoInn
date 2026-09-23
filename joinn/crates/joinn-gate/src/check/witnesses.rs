//! Check 3: founding witnesses replay exhaustively.

mod restrict_if_needed;

use crate::oracle::{Catching, Oracle};
use joinn_dna::{Cell, Witness};
use joinn_frame::{CheckId, CounterExample, FrameRef, FrameRegistry, Refusal, Subject, Verdict};
use std::collections::BTreeMap;

/// Replay every founding witness. A failure names the witness.
pub fn check(
    cell: &Cell,
    frames: &FrameRegistry,
    oracle: &dyn Oracle,
    allele_frame: FrameRef,
    seed: u64,
) -> Verdict<()> {
    for (i, w) in cell.coding.founding.iter().enumerate() {
        match replay_one(w, frames, oracle, allele_frame, seed) {
            Verdict::Ok(()) => {}
            Verdict::Refused(mut r) => {
                r.reason = format!("founding witness {i}: {}", r.reason);
                r.subject = Subject::Witness;
                return Verdict::Refused(r);
            }
        }
    }
    Verdict::Ok(())
}

fn replay_one(
    w: &Witness,
    frames: &FrameRegistry,
    oracle: &dyn Oracle,
    allele_frame: FrameRef,
    seed: u64,
) -> Verdict<()> {
    let mut inputs = BTreeMap::new();
    for (pos, v) in &w.inputs {
        match coerce(v, allele_frame, frames) {
            Verdict::Ok(c) => {
                inputs.insert(*pos, c);
            }
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
    }
    let caught = Catching { inner: oracle };
    let outs = match caught.apply(&inputs) {
        Verdict::Ok(o) => o,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    for (pos, expected) in &w.outputs {
        let Some(got) = outs.get(pos) else {
            return Verdict::Refused(Refusal {
                check: CheckId::Witnesses,
                subject: Subject::Witness,
                reason: format!("missing out-port {pos}"),
                counterexample: None,
                seed,
            });
        };
        let expected = match coerce(expected, *got.frame(), frames) {
            Verdict::Ok(v) => v,
            Verdict::Refused(_) => expected.clone(),
        };
        // Restriction: if the allele is wider, ρ back to the expected frame.
        let comparable = match restrict_if_needed::restrict_if_needed(got, expected.frame(), frames)
        {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let eq = frames
            .get(expected.frame())
            .is_some_and(|f| f.eq(&expected, &comparable));
        if !eq {
            return Verdict::Refused(Refusal {
                check: CheckId::Witnesses,
                subject: Subject::Witness,
                reason: format!("witness out-port {pos} does not replay"),
                counterexample: Some(CounterExample {
                    bindings: w
                        .inputs
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.clone()))
                        .collect(),
                    expected: Some(expected),
                    got: Some(got.clone()),
                }),
                seed,
            });
        }
    }
    Verdict::Ok(())
}

fn coerce(
    v: &joinn_frame::Value,
    target: FrameRef,
    frames: &FrameRegistry,
) -> Verdict<joinn_frame::Value> {
    if v.frame() == &target {
        return Verdict::Ok(v.clone());
    }
    let Some(dest) = frames.get(&target) else {
        return Verdict::Ok(v.clone());
    };
    dest.embed(v.frame(), v)
}
