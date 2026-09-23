//! Refuse a body that reads a signal its host does not emit.

use joinn_dna::Body;
use joinn_frame::{CheckId, Refusal, Verdict};

use super::Signals;

/// A body reading an undeclared signal is refused, naming the signal.
pub fn check_signals(body: &Body, host: &Signals) -> Verdict<()> {
    for name in &body.coding.reads {
        if !host.emits(name) {
            return Verdict::Refused(Refusal::structural(
                CheckId::Other,
                format!("signal {name} is not declared by this host"),
            ));
        }
    }
    Verdict::Ok(())
}
