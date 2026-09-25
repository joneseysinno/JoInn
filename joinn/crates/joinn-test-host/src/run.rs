//! Drive a body under the test host with scripted events.

use joinn_dna::{Body, Cell};
use joinn_frame::{Hash, Verdict};
use joinn_gate::NativeRegistry;
use joinn_host::{Host, check_signals, describe, describe_refusal, intent_set};
use joinn_live::BodyState;
use std::collections::BTreeMap;

use crate::capture::Capture;
use crate::host::TestHost;
use crate::raw_event::RawEvent;
use crate::value_of::value_of;

/// Drive a body under the test host with scripted events.
pub fn run(
    body: Body,
    cells: BTreeMap<Hash, Cell>,
    natives: NativeRegistry,
    events: Vec<RawEvent>,
) -> Verdict<Capture> {
    let mut host = TestHost::new(body.clone(), cells.clone());
    match check_signals(&body, host.signals()) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Verdict::Refused(r),
    }
    let ports = intent_set(&body, &cells);
    let mut intent_set = BTreeMap::new();
    // A lone body has no universe alias. The set is that body's in-ports.
    intent_set.insert(String::new(), ports);
    let mut state = match BodyState::new(body, cells, natives, 1) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    for (epoch, raw) in events.into_iter().enumerate() {
        let intent = match host.intend(raw) {
            Verdict::Ok(Some(i)) => i,
            Verdict::Ok(None) => continue,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let value = match value_of(intent.term) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        match state.inject(
            &intent.address.instance,
            intent.address.port,
            value,
            epoch as u64,
        ) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
        match state.run() {
            Verdict::Ok(reports) => {
                for report in reports {
                    let Some(name) = report.fired else {
                        continue;
                    };
                    match describe(&state, &name) {
                        Verdict::Ok(d) => match host.present(&d) {
                            Verdict::Ok(()) => {}
                            Verdict::Refused(r) => return Verdict::Refused(r),
                        },
                        Verdict::Refused(r) => return Verdict::Refused(r),
                    }
                }
            }
            Verdict::Refused(r) => {
                let d = describe_refusal(&state, &intent.address.instance, &r.reason);
                match host.present(&d) {
                    Verdict::Ok(()) => {}
                    Verdict::Refused(e) => return Verdict::Refused(e),
                }
            }
        }
    }
    Verdict::Ok(Capture {
        descriptions: host.captures,
        far_side: Vec::new(),
        intent_set,
    })
}
