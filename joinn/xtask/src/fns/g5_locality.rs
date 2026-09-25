//! Gate 5 item 8: the far side does not carry the refusing body's words.

use super::{load_phase5_bodies, load_universe_file, text_val, workspace_root};
use joinn_frame::Verdict;
use joinn_host::probe;
use joinn_link::{Address, UniverseReport, UniverseState, bind, format_link_refusal};
use std::fs;

pub(crate) fn g5_locality() -> bool {
    let Ok(root) = workspace_root() else {
        return false;
    };
    let Ok(secret) = fs::read_to_string(
        root.join("corpus")
            .join("phase5")
            .join("controls")
            .join("inner_reason.txt"),
    ) else {
        return false;
    };
    let secret = secret.trim();
    if secret.is_empty() {
        return false;
    }
    let Ok(u) = load_universe_file("phase5/universe.universe") else {
        return false;
    };
    let Ok(store) = load_phase5_bodies() else {
        return false;
    };
    let Verdict::Ok(bound) = bind(&u, &store) else {
        return false;
    };
    let mut state = match UniverseState::new(&u, &bound, joinn_prim::sealed_natives()) {
        Verdict::Ok(s) => s,
        Verdict::Refused(_) => return false,
    };
    let Ok(two) = text_val("2") else {
        return false;
    };
    let Ok(three) = text_val("3") else {
        return false;
    };
    let Ok(again_a) = text_val("2") else {
        return false;
    };
    let Ok(again_b) = text_val("3") else {
        return false;
    };
    for (i, (instance, value)) in [("cli_a", two), ("cli_b", three)].into_iter().enumerate() {
        let addr = Address {
            instance: instance.into(),
            port: 0,
        };
        if !matches!(
            state.inject("calc", &addr, value, i as u64),
            Verdict::Ok(())
        ) {
            return false;
        }
    }
    if matches!(state.run(), Verdict::Refused(_)) {
        return false;
    }
    for (i, (instance, value)) in [("cli_a", again_a), ("cli_b", again_b)]
        .into_iter()
        .enumerate()
    {
        let addr = Address {
            instance: instance.into(),
            port: 0,
        };
        if !matches!(
            state.inject("calc", &addr, value, i as u64),
            Verdict::Ok(())
        ) {
            return false;
        }
    }
    let reports = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(_) => return false,
    };
    let far_lines: Vec<String> = reports
        .iter()
        .filter_map(|r| match r {
            UniverseReport::Link(refusal) => Some(format_link_refusal(refusal)),
            _ => None,
        })
        .collect();
    if far_lines.is_empty() {
        return false;
    }
    if far_lines.iter().any(|line| line.contains(secret)) {
        return false;
    }
    let Some(body) = state.body("units") else {
        return false;
    };
    match probe(
        body,
        Address {
            instance: "scale".into(),
            port: 0,
        },
    ) {
        Verdict::Ok(d) => d.label.contains(secret),
        Verdict::Refused(_) => false,
    }
}
