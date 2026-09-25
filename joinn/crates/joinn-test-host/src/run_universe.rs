//! Drive a universe under the test host. Capture what each body presented.

use joinn_frame::Verdict;
use joinn_gate::NativeRegistry;
use joinn_host::describe;
use joinn_link::{Bound, Universe, UniverseReport, UniverseState};
use std::collections::BTreeMap;

use crate::capture::Capture;
use crate::host_intent_set::host_intent_set;
use crate::raw_event::RawEvent;
use crate::value_of::value_of;

/// Inject each round, run, and collect reports from every round.
pub fn run_universe(
    universe: &Universe,
    bound: Bound,
    natives: NativeRegistry,
    rounds: Vec<Vec<(String, RawEvent)>>,
) -> Verdict<Capture> {
    let mut state = match UniverseState::new(universe, &bound, natives) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let mut descriptions = Vec::new();
    let mut far_side = Vec::new();
    let mut epoch = 0u64;
    for round in rounds {
        for (alias, raw) in round {
            let value = match value_of(raw.term) {
                Verdict::Ok(v) => v,
                Verdict::Refused(r) => return Verdict::Refused(r),
            };
            match state.inject(&alias, &raw.address, value, epoch) {
                Verdict::Ok(()) => {}
                Verdict::Refused(r) => return Verdict::Refused(r),
            }
            epoch = epoch.saturating_add(1);
        }
        let reports = match state.run() {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
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
    }
    let mut intent_set = BTreeMap::new();
    for (alias, (body, cells)) in bound.iter() {
        intent_set.insert(
            alias.to_owned(),
            host_intent_set(universe, alias, body, cells),
        );
    }
    Verdict::Ok(Capture {
        descriptions,
        far_side,
        intent_set,
    })
}
