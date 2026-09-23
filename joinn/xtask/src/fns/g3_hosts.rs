//! Gate 3 item 1: the calculator's result is the same under both hosts.

use super::{load_calculator, run_calculator_bin};
use joinn_frame::{Term, Verdict};
use joinn_host::Address;
use joinn_test_host::RawEvent;

pub(crate) fn g3_hosts() -> bool {
    let Ok((body, cells)) = load_calculator() else {
        return false;
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
        return false;
    };
    let Some(sum) = cap.descriptions.iter().find(|d| d.instance == "sum") else {
        return false;
    };
    let test_has_five = sum.ports.iter().any(|p| p.value.as_deref() == Some("5"));
    let Ok(cli) = run_calculator_bin() else {
        return false;
    };
    test_has_five && cli.contains("2 + 3 = 5")
}
