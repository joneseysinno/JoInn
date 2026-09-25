//! `cargo xtask probe-refusal` — double delivery; probe label, then far-side lines.

use super::{load_phase5_bodies, load_universe_file, text_val};
use joinn_frame::Verdict;
use joinn_host::probe;
use joinn_link::{Address, UniverseReport, UniverseState, bind, format_link_refusal};

/// Run the double-delivery scenario; print probe label, then each far-side line.
pub(crate) fn probe_refusal() -> Result<(), String> {
    let u = load_universe_file("phase5/universe.universe")?;
    let store = load_phase5_bodies()?;
    let bound = match bind(&u, &store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let mut state = match UniverseState::new(&u, &bound, joinn_prim::sealed_natives()) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let two = text_val("2")?;
    let three = text_val("3")?;
    let again_a = text_val("2")?;
    let again_b = text_val("3")?;
    for (i, (instance, value)) in [("cli_a", two), ("cli_b", three)].into_iter().enumerate() {
        let addr = Address {
            instance: instance.into(),
            port: 0,
        };
        match state.inject("calc", &addr, value, i as u64) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Err(r.reason),
        }
    }
    match state.run() {
        Verdict::Ok(_) => {}
        Verdict::Refused(r) => return Err(r.reason),
    }
    for (i, (instance, value)) in [("cli_a", again_a), ("cli_b", again_b)]
        .into_iter()
        .enumerate()
    {
        let addr = Address {
            instance: instance.into(),
            port: 0,
        };
        match state.inject("calc", &addr, value, i as u64) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Err(r.reason),
        }
    }
    let reports = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let body = state
        .body("units")
        .ok_or_else(|| "units body missing".to_string())?;
    let label = match probe(
        body,
        Address {
            instance: "scale".into(),
            port: 0,
        },
    ) {
        Verdict::Ok(d) => d.label,
        Verdict::Refused(r) => return Err(r.reason),
    };
    println!("{label}");
    for report in &reports {
        if let UniverseReport::Link(refusal) = report {
            println!("{}", format_link_refusal(refusal));
        }
    }
    Ok(())
}
