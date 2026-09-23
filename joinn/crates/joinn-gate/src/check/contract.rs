//! Check 1: ports well-formed, frames registered, declarations empty.

use joinn_dna::{Cell, Direction};
use joinn_frame::{CheckId, FrameRegistry, Refusal, Subject, Verdict};
use std::collections::BTreeSet;

/// Structural contract check. A refusal here may lack a counter-example.
pub fn check(cell: &Cell, frames: &FrameRegistry) -> Verdict<()> {
    if !cell.coding.declarations.is_empty() {
        return Verdict::Refused(Refusal::structural(
            CheckId::Contract,
            "declarations are not implemented",
        ));
    }
    let mut seen = BTreeSet::new();
    let retired: BTreeSet<u32> = cell.coding.contract.retired.iter().copied().collect();
    let mut outs = 0u32;
    for port in &cell.coding.contract.ports {
        if !seen.insert(port.position) {
            return Verdict::Refused(Refusal::structural(
                CheckId::Contract,
                format!("duplicate port position {}", port.position),
            ));
        }
        if retired.contains(&port.position) {
            return Verdict::Refused(Refusal::structural(
                CheckId::Contract,
                format!("live port {} is retired", port.position),
            ));
        }
        if !frames.contains(&port.frame) {
            return Verdict::Refused(Refusal::structural(
                CheckId::Contract,
                format!(
                    "unregistered frame {} on port {}",
                    port.frame, port.position
                ),
            ));
        }
        if port.direction == Direction::Out {
            outs += 1;
        }
    }
    if outs == 0 {
        return Verdict::Refused(Refusal::structural(
            CheckId::Contract,
            "contract has no out-port",
        ));
    }
    if !frames.contains(&cell.coding.frame) {
        return Verdict::Refused(Refusal {
            check: CheckId::Contract,
            subject: Subject::Frame(cell.coding.frame.to_string()),
            reason: format!("coding frame {} is not registered", cell.coding.frame),
            counterexample: None,
            seed: 0,
        });
    }
    Verdict::Ok(())
}
