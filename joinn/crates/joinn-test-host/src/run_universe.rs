//! Drive a universe under the test host. Capture what each body presented.

use joinn_frame::Verdict;
use joinn_gate::NativeRegistry;
use joinn_host::describe;
use joinn_link::{Bound, Universe, UniverseReport, UniverseState, grant};

use crate::capture::Capture;
use crate::raw_event::RawEvent;
use crate::value_of::value_of;

/// Inject scripted events, grant one ordered link, run, capture fire descriptions.
pub fn run_universe(
    universe: &Universe,
    bound: Bound,
    natives: NativeRegistry,
    grant_to: Option<(&str, &str)>,
    events: Vec<(String, RawEvent)>,
) -> Verdict<Capture> {
    let mut state = match UniverseState::new(universe, &bound, natives) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    if let Some((link, body)) = grant_to {
        match grant(state.link_runtime(), universe, link, body) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
    }
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
    for report in reports {
        let UniverseReport::Fired { body, instance } = report else {
            continue;
        };
        let Some(live) = state.body(&body) else {
            continue;
        };
        match describe(live, &instance) {
            Verdict::Ok(d) => descriptions.push(d),
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
    }
    Verdict::Ok(Capture {
        descriptions,
        intent_set: Default::default(),
    })
}
