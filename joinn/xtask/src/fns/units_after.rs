//! Drive the linked universe and read what units did.

use super::{int_val, load_phase5_bodies, text_val};
use joinn_frame::Verdict;
use joinn_host::describe;
use joinn_link::{bind_bodies, grant, Address, Universe, UniverseReport, UniverseState};

pub(crate) fn units_after(
    universe: &Universe,
    grant_link: Option<&str>,
) -> Result<(usize, Option<String>), String> {
    let supplied = load_phase5_bodies()?;
    let bound = match bind_bodies(universe, &supplied) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let mut state = match UniverseState::new(universe, bound, joinn_prim::sealed_natives()) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(r.reason),
    };
    if let Some(link_id) = grant_link {
        match grant(state.link_runtime(), universe, link_id, "units") {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Err(r.reason),
        }
    }
    let a = Address {
        instance: "cli_a".into(),
        port: 0,
    };
    let b = Address {
        instance: "cli_b".into(),
        port: 0,
    };
    let factor = Address {
        instance: "scale".into(),
        port: 1,
    };
    match state.inject("calc", &a, text_val("2")?, 0) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Err(r.reason),
    }
    match state.inject("calc", &b, text_val("3")?, 1) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Err(r.reason),
    }
    match state.inject("units", &factor, int_val(12)?, 2) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Err(r.reason),
    }
    let reports = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let fires = reports
        .iter()
        .filter(|r| matches!(r, UniverseReport::Fired { body, .. } if body == "units"))
        .count();
    let scale2 = match state.body("units") {
        Some(body) => match describe(body, "scale") {
            Verdict::Ok(d) => d
                .ports
                .iter()
                .find(|p| p.position == 2)
                .and_then(|p| p.value.clone()),
            Verdict::Refused(r) => return Err(r.reason),
        },
        None => None,
    };
    Ok((fires, scale2))
}
