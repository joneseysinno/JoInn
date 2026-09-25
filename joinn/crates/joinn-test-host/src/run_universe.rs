//! Drive a universe under the test host. Capture what each body presented.

use joinn_frame::Verdict;
use joinn_gate::NativeRegistry;
use joinn_host::describe;
use joinn_link::{Bound, Universe, UniverseReport, UniverseState};

use crate::capture::Capture;
use crate::host_intent_set::host_intent_set;
use crate::raw_event::RawEvent;
use crate::value_of::value_of;

/// Inject scripted events, run, capture fire descriptions and far-side refusals.
pub fn run_universe(
    universe: &Universe,
    bound: Bound,
    natives: NativeRegistry,
    events: Vec<(String, RawEvent)>,
) -> Verdict<Capture> {
    let mut state = match UniverseState::new(universe, &bound, natives) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    for (epoch, (alias, raw)) in events.into_iter().enumerate() {
        let value = match value_of(raw.term) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        match state.inject(&alias, &raw.address, value, epoch as u64) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
    }
    let reports = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let mut descriptions = Vec::new();
    let mut far_side = Vec::new();
    for report in reports {
        match report {
            UniverseReport::Fired { body, instance } => {
                let Some(live) = state.body(&body) else {
                    continue;
                };
                match describe(live, &instance) {
                    Verdict::Ok(d) => descriptions.push(d),
                    Verdict::Refused(r) => return Verdict::Refused(r),
                }
            }
            UniverseReport::Link(refusal) => far_side.push(refusal),
            UniverseReport::Refused { .. } => {}
        }
    }
    // Done-when: intent_set for units is {scale@1}. Prefer units, else meters.
    let intent_set = ["units", "meters"]
        .iter()
        .find_map(|alias| {
            bound
                .get(alias)
                .map(|(body, cells)| host_intent_set(universe, alias, body, cells))
        })
        .unwrap_or_default();
    Verdict::Ok(Capture {
        descriptions,
        far_side,
        intent_set,
    })
}
