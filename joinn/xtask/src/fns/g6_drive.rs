//! §2.12's script on one body: deltas, the V121 bound, and regrow equality.

use std::collections::BTreeMap;

use joinn_dna::{Body, Cell};
use joinn_frame::{Hash, Verdict};
use joinn_link::instance_ports;
use joinn_live::BodyState;
use joinn_visual::{Delta, Scene, table_bytes};

use super::regrow::{Form, cleared_elsewhere, event};

/// §2.12: refused at the membrane, then `2`, then `3` (`sum` fires).
const SCRIPT: [(&str, &str); 3] = [("cli_a", "two"), ("cli_a", "2"), ("cli_b", "3")];

/// One drawn moment: `grow` has no delta; each event's delta is what `apply` writes.
pub(crate) struct G6Step {
    pub label: String,
    pub scene: Scene,
    pub delta: Option<Delta>,
}

/// The delta-built scene after the script. `failures` are bound or regrow breaks;
/// a host refusal is `Err` instead, because there is no scene to judge.
pub(crate) struct G6Driven {
    pub steps: Vec<G6Step>,
    pub scene: Scene,
    pub state: BodyState,
    pub failures: Vec<String>,
}

/// Grow, then each script event through `check_intent`, inject, `run`, and
/// `apply_run`. Tables after every event are compared with `regrow`. `body` is
/// the wired body, or a contact's lowered body; the bound counts drawn ports.
pub(crate) fn g6_drive(
    form: Form<'_>,
    body: &Body,
    cells: &BTreeMap<Hash, Cell>,
) -> Result<G6Driven, String> {
    let mut scene = match form.grow(body, cells) {
        Verdict::Ok(scene) => scene,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let mut state =
        match BodyState::new(body.clone(), cells.clone(), joinn_prim::sealed_natives(), 1) {
            Verdict::Ok(state) => state,
            Verdict::Refused(r) => return Err(r.reason),
        };
    let ports: BTreeMap<String, usize> = match form {
        Form::Wired => match instance_ports(body, cells) {
            Verdict::Ok(map) => map.into_iter().map(|(name, ps)| (name, ps.len())).collect(),
            Verdict::Refused(r) => return Err(r.reason),
        },
        Form::Contact(_) => {
            let mut counts = BTreeMap::new();
            for p in &scene.layout().ports {
                *counts.entry(p.address.instance.clone()).or_default() += 1;
            }
            counts
        }
    };
    let mut steps = vec![G6Step {
        label: "grow".to_owned(),
        scene: scene.clone(),
        delta: None,
    }];
    let mut failures = Vec::new();
    for (epoch, (instance, text)) in (0u64..).zip(SCRIPT) {
        let label = format!("event {} {text:?}", epoch + 1);
        let before = scene.tables().clone();
        let (delta, touched) = event(
            &mut scene,
            &mut state,
            (body, cells),
            (instance, text),
            epoch,
        )?;
        let extra = cleared_elsewhere(&scene, &before, &touched);
        let mut base = 0usize;
        for name in &touched {
            match ports.get(name) {
                Some(n) => base += 1 + n,
                None => failures.push(format!(
                    "{label}: touched {name} has no ports; acceptance is an instance in the port map"
                )),
            }
        }
        let bound = base + extra;
        if delta.rows.len() > bound {
            failures.push(format!(
                "{label}: {} row(s) break the V121 bound of {bound}",
                delta.rows.len()
            ));
        }
        let regrown = match form.regrow(body, cells, &state) {
            Verdict::Ok(scene) => scene,
            Verdict::Refused(r) => return Err(r.reason),
        };
        if table_bytes(scene.tables()) != table_bytes(regrown.tables()) {
            failures.push(format!(
                "{label}: delta-built tables differ from regrow (V122)"
            ));
        }
        let pending = scene.take_pending().unwrap_or_default();
        if pending != delta {
            failures.push(format!("{label}: the pending delta is not the run's delta"));
        }
        if scene.take_pending().is_some() {
            failures.push(format!(
                "{label}: rows still pending after the delta was taken (V12)"
            ));
        }
        steps.push(G6Step {
            label,
            scene: scene.clone(),
            delta: Some(delta),
        });
    }
    Ok(G6Driven {
        steps,
        scene,
        state,
        failures,
    })
}
