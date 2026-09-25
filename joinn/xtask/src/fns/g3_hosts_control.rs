//! Gate 3 item 1 control: a sum of 4 must not be what both hosts produced.

use super::load_calculator;
use joinn_frame::{Term, Verdict};
use joinn_host::Address;
use joinn_test_host::RawEvent;

pub(crate) fn g3_hosts_control(_art: &()) -> bool {
    let Ok((body, cells)) = load_calculator() else {
        return true;
    };
    let natives = joinn_prim::sealed_natives();
    let events = vec![
        RawEvent {
            address: Address {
                instance: "cli_a".into(),
                port: 0,
            },
            term: Term::text("2"),
        },
        RawEvent {
            address: Address {
                instance: "cli_b".into(),
                port: 0,
            },
            term: Term::text("3"),
        },
    ];
    let Verdict::Ok(cap) = joinn_test_host::run(body, cells, natives, events) else {
        return true;
    };
    let Some(sum) = cap.descriptions.iter().find(|d| d.instance == "sum") else {
        return true;
    };
    sum.ports.iter().any(|p| p.value.as_deref() == Some("4"))
}
