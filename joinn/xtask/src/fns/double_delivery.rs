//! Double delivery: two rounds of calc `2`, `3`. Probe label and the test host's far side.

use super::{load_phase5_bodies, load_universe_file, text_val};
use joinn_frame::{Term, Verdict};
use joinn_host::{Address, probe};
use joinn_link::{LinkRefusal, UniverseState, bind};
use joinn_test_host::{RawEvent, run_universe};

/// Probe label on units, and the far-side refusals the test host recorded.
pub(crate) struct DoubleDelivery {
    /// `probe` label at `units.scale@0`.
    pub label: String,
    /// Far side from `Capture.far_side`. No body reason strings.
    pub far_side: Vec<LinkRefusal>,
}

/// Run the double-delivery scenario once for the test host and once to probe.
pub(crate) fn double_delivery() -> Result<DoubleDelivery, String> {
    let universe = load_universe_file("phase5/universe.universe")?;
    let store = load_phase5_bodies()?;
    let round = vec![
        (
            "calc".to_owned(),
            RawEvent {
                address: Address {
                    instance: "cli_a".into(),
                    port: 0,
                },
                term: Term::text("2"),
            },
        ),
        (
            "calc".to_owned(),
            RawEvent {
                address: Address {
                    instance: "cli_b".into(),
                    port: 0,
                },
                term: Term::text("3"),
            },
        ),
    ];
    let rounds = vec![round.clone(), round];
    let bound = match bind(&universe, &store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let captured = match run_universe(
        &universe,
        bound,
        joinn_prim::sealed_natives(),
        rounds.clone(),
    ) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let bound = match bind(&universe, &store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let mut state = match UniverseState::new(&universe, &bound, joinn_prim::sealed_natives()) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let mut epoch = 0u64;
    for round in &rounds {
        for (alias, raw) in round {
            let Term::Text(text) = &raw.term else {
                return Err("double delivery sends text".into());
            };
            let value = text_val(text)?;
            match state.inject(alias, &raw.address, value, epoch) {
                Verdict::Ok(()) => {}
                Verdict::Refused(r) => return Err(r.reason),
            }
            epoch = epoch.saturating_add(1);
        }
        match state.run() {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) => return Err(r.reason),
        }
    }
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
    Ok(DoubleDelivery {
        label,
        far_side: captured.far_side,
    })
}
