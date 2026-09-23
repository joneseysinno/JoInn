//! Gate 5.1 item 2: both hosts run the universe, and the calculator is a prefix.

use super::{load_phase5_bodies, run_body_bin, workspace_root};
use joinn_frame::{Term, Verdict};
use joinn_host::Address;
use joinn_link::{bind_bodies, parse_universe};
use joinn_test_host::RawEvent;
use std::fs;

pub(crate) fn g51_hosts() -> bool {
    let Ok(cli) = run_body_bin("universe", b"two\n2\n3\n12\n") else {
        return false;
    };
    let Ok(root) = workspace_root() else {
        return false;
    };
    let transcripts = root.join("corpus").join("transcripts");
    let Ok(want) = fs::read_to_string(transcripts.join("universe.txt")) else {
        return false;
    };
    let Ok(calc) = fs::read(transcripts.join("calculator.txt")) else {
        return false;
    };
    if cli != want {
        return false;
    }
    let got = cli.as_bytes();
    if got.len() < calc.len() || &got[..calc.len()] != calc.as_slice() {
        return false;
    }
    let Ok(src) = fs::read_to_string(root.join("corpus").join("phase5").join("universe.universe"))
    else {
        return false;
    };
    let Verdict::Ok(universe) = parse_universe(&src) else {
        return false;
    };
    let Ok(supplied) = load_phase5_bodies() else {
        return false;
    };
    let Verdict::Ok(bound) = bind_bodies(&universe, &supplied) else {
        return false;
    };
    let events = vec![
        (
            "calc".into(),
            RawEvent {
                address: Address {
                    instance: "cli_a".into(),
                    port: 0,
                },
                term: Term::text("2"),
            },
        ),
        (
            "calc".into(),
            RawEvent {
                address: Address {
                    instance: "cli_b".into(),
                    port: 0,
                },
                term: Term::text("3"),
            },
        ),
        (
            "units".into(),
            RawEvent {
                address: Address {
                    instance: "scale".into(),
                    port: 1,
                },
                term: Term::int(12),
            },
        ),
    ];
    let Verdict::Ok(cap) = joinn_test_host::run_universe(
        &universe,
        bound,
        joinn_prim::sealed_natives(),
        Some(("e0", "units")),
        events,
    ) else {
        return false;
    };
    cap.descriptions.iter().any(|d| {
        d.instance == "scale" && d.ports.iter().any(|p| p.position == 2 && p.value.as_deref() == Some("60"))
    })
}
