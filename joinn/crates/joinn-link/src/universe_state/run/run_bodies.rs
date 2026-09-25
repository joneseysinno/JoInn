//! One canonical pass over every bound body alias.

use joinn_frame::{Value, Verdict};
use std::collections::BTreeMap;

use crate::Address;
use crate::universe_state::{UniverseReport, UniverseState};

use crate::universe_state::run::push_body_refusal::push_body_refusal;

pub(in crate::universe_state::run) fn run_bodies(
    state: &mut UniverseState,
    aliases: &[String],
    link_refusal_on_body_refuse: bool,
    emitted: &mut BTreeMap<(String, String, u32), Value>,
    changed: &mut bool,
    reports: &mut Vec<UniverseReport>,
    delivered_this_pass: &mut BTreeMap<(String, String, u32), (String, Address)>,
) {
    for alias in aliases {
        let used = state.bodies.get(alias).map(|b| b.steps()).unwrap_or(0);
        let remaining = state.budget.saturating_sub(state.spent);
        if let Some(body) = state.bodies.get_mut(alias) {
            body.set_budget(used.saturating_add(remaining));
        }
        let ran = match state.bodies.get_mut(alias) {
            Some(body) => body.run(),
            None => continue,
        };
        let now = state.bodies.get(alias).map(|b| b.steps()).unwrap_or(used);
        state.spent = state.spent.saturating_add(now.saturating_sub(used));
        match ran {
            Verdict::Ok(steps) => {
                for step in steps {
                    let Some(instance) = step.fired else {
                        continue;
                    };
                    *changed = true;
                    reports.push(UniverseReport::Fired {
                        body: alias.clone(),
                        instance: instance.clone(),
                    });
                    if let Some(ports) = state
                        .bodies
                        .get(alias)
                        .and_then(|b| b.last_ports(&instance))
                    {
                        for (port, value) in ports {
                            emitted.insert((alias.clone(), instance.clone(), *port), value.clone());
                        }
                    }
                }
            }
            Verdict::Refused(r) => {
                push_body_refusal(
                    alias,
                    &r,
                    state,
                    reports,
                    link_refusal_on_body_refuse,
                    delivered_this_pass,
                );
            }
        }
    }
}
