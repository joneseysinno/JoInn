//! Gate 5.2 item 2: a body refusal is a report inside the universe.

use super::{load_phase5_bodies, run_body_bin, text_val, workspace_root};
use joinn_frame::Verdict;
use joinn_host::Address;
use joinn_link::{UniverseReport, UniverseState, bind, parse_universe};
use std::fs;

pub(crate) fn g52_refusal() -> bool {
    let Ok(cli) = run_body_bin("universe", b"two\n2\n3\n12\n") else {
        return false;
    };
    let Ok(root) = workspace_root() else {
        return false;
    };
    let Ok(want) = fs::read_to_string(root.join("corpus").join("transcripts").join("universe.txt"))
    else {
        return false;
    };
    if cli != want {
        return false;
    }
    if !cli.contains("two") || !cli.contains("refused") {
        return false;
    }
    let Ok(src) = fs::read_to_string(root.join("corpus").join("phase5").join("universe.universe"))
    else {
        return false;
    };
    let Verdict::Ok(universe) = parse_universe(&src) else {
        return false;
    };
    let Ok(store) = load_phase5_bodies() else {
        return false;
    };
    let Verdict::Ok(bound) = bind(&universe, &store) else {
        return false;
    };
    let mut state = match UniverseState::new(&universe, &bound, joinn_prim::sealed_natives()) {
        Verdict::Ok(s) => s,
        Verdict::Refused(_) => return false,
    };
    let addr = Address {
        instance: "cli_a".into(),
        port: 0,
    };
    let Ok(text) = text_val("two") else {
        return false;
    };
    match state.inject("calc", &addr, text, 0) {
        Verdict::Ok(()) => {}
        Verdict::Refused(_) => return false,
    }
    let reports = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(_) => return false,
    };
    reports.iter().any(|r| {
        matches!(
            r,
            UniverseReport::Refused { body, instance }
                if body == "calc" && instance == "cli_a"
        )
    })
}
