//! Drive a contact body under the test host: admit, lower, then run.

use joinn_dna::{Cell, Contact};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_gate::NativeRegistry;
use joinn_link::lower;
use std::collections::BTreeMap;

use crate::capture::Capture;
use crate::raw_event::RawEvent;
use crate::run::run;

/// A refused contact is the refusal; an admitted one runs as the body `lower`
/// derives from it, on the same path as a wired body.
pub fn run_contact(
    contact: &Contact,
    cells: BTreeMap<Hash, Cell>,
    natives: NativeRegistry,
    events: Vec<RawEvent>,
) -> Verdict<Capture> {
    let body = match lower(contact, &cells, &FrameRegistry::phase1()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    run(body, cells, natives, events)
}
