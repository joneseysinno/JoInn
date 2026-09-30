//! The §2.12 script as test-host events.

use joinn_frame::Term;
use joinn_host::Address;
use joinn_test_host::RawEvent;

use super::SCRIPT;

pub(super) fn script_events() -> Vec<RawEvent> {
    SCRIPT
        .iter()
        .map(|(instance, port, line)| RawEvent {
            address: Address {
                instance: (*instance).into(),
                port: *port,
            },
            term: Term::text(*line),
        })
        .collect()
}
