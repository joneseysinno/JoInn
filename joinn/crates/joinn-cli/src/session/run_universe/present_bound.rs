//! Present fires, body refusals, and link refusals for a bound universe.

use crate::load::value_in_frame;
use crate::present::prompt;
use crate::session::in_ports::in_ports;
use crate::session::read_line::read_line;
use joinn_dna::Body;
use joinn_frame::Verdict;
use joinn_link::{Address, Bound, Mark, Universe, UniverseReport, UniverseState};
use std::collections::{BTreeMap, BTreeSet};
use std::io;

use super::append_reports::append_reports;

/// Prompt unlinked ports, inject every line into the universe, present reports.
pub(super) fn present_bound(
    universe: &Universe,
    bound: &Bound,
    lines: &mut impl Iterator<Item = io::Result<String>>,
) -> Result<String, String> {
    let faces: BTreeMap<String, Body> = bound
        .iter()
        .map(|(alias, (body, _))| (alias.to_owned(), body.clone()))
        .collect();
    let mut state = match UniverseState::new(universe, bound, joinn_prim::sealed_natives()) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let mut fed = BTreeSet::new();
    for link in &universe.coding.links {
        for member in &link.members {
            if member.mark == Mark::Head {
                fed.insert((member.body.clone(), member.instance.clone(), member.port));
            }
        }
    }
    let mut groups: Vec<(String, Vec<joinn_link::BoundaryPort>)> = Vec::new();
    for (alias, (body, cells)) in bound.iter() {
        let ports = in_ports(body, cells)
            .into_iter()
            .filter(|port| {
                !fed.contains(&(
                    alias.to_owned(),
                    port.address.instance.clone(),
                    port.address.port,
                ))
            })
            .collect::<Vec<_>>();
        if !ports.is_empty() {
            groups.push((alias.to_owned(), ports));
        }
    }
    let mut out = String::new();
    let mut epoch = 0u64;
    for (index, (alias, ports)) in groups.iter().enumerate() {
        let Some(body) = faces.get(alias) else {
            continue;
        };
        if index == 0 {
            let first = read_line(lines)?;
            out.push_str(prompt(body, &ports[0].address.instance, "> "));
            out.push_str(&first);
            out.push('\n');
            match state.inject(
                alias,
                &Address {
                    instance: ports[0].address.instance.clone(),
                    port: ports[0].address.port,
                },
                value_in_frame(&ports[0].frame, &first)?,
                epoch,
            ) {
                Verdict::Ok(()) => {}
                Verdict::Refused(r) => return Err(r.reason),
            }
            epoch = epoch.saturating_add(1);
            let early = match state.run() {
                Verdict::Ok(rs) => rs,
                Verdict::Refused(r) => return Err(r.reason),
            };
            let refused = early.iter().any(|r| {
                matches!(
                    r,
                    UniverseReport::Refused { body: b, instance }
                        if b == alias && instance == &ports[0].address.instance
                )
            });
            append_reports(&mut out, &early, &state, &faces)?;
            if refused {
                for port in ports {
                    let line = read_line(lines)?;
                    out.push_str(prompt(body, &port.address.instance, "> "));
                    out.push_str(&line);
                    out.push('\n');
                    match state.inject(
                        alias,
                        &Address {
                            instance: port.address.instance.clone(),
                            port: port.address.port,
                        },
                        value_in_frame(&port.frame, &line)?,
                        epoch,
                    ) {
                        Verdict::Ok(()) => {}
                        Verdict::Refused(r) => return Err(r.reason),
                    }
                    epoch = epoch.saturating_add(1);
                }
                let reports = match state.run() {
                    Verdict::Ok(rs) => rs,
                    Verdict::Refused(r) => return Err(r.reason),
                };
                append_reports(&mut out, &reports, &state, &faces)?;
            } else {
                for port in ports.iter().skip(1) {
                    let line = read_line(lines)?;
                    out.push_str(prompt(body, &port.address.instance, "> "));
                    out.push_str(&line);
                    out.push('\n');
                    match state.inject(
                        alias,
                        &Address {
                            instance: port.address.instance.clone(),
                            port: port.address.port,
                        },
                        value_in_frame(&port.frame, &line)?,
                        epoch,
                    ) {
                        Verdict::Ok(()) => {}
                        Verdict::Refused(r) => return Err(r.reason),
                    }
                    epoch = epoch.saturating_add(1);
                }
                let reports = match state.run() {
                    Verdict::Ok(rs) => rs,
                    Verdict::Refused(r) => return Err(r.reason),
                };
                append_reports(&mut out, &reports, &state, &faces)?;
            }
        } else {
            for port in ports {
                let line = read_line(lines)?;
                out.push_str(prompt(body, &port.address.instance, "> "));
                out.push_str(&line);
                out.push('\n');
                match state.inject(
                    alias,
                    &Address {
                        instance: port.address.instance.clone(),
                        port: port.address.port,
                    },
                    value_in_frame(&port.frame, &line)?,
                    epoch,
                ) {
                    Verdict::Ok(()) => {}
                    Verdict::Refused(r) => return Err(r.reason),
                }
                epoch = epoch.saturating_add(1);
            }
            let reports = match state.run() {
                Verdict::Ok(rs) => rs,
                Verdict::Refused(r) => return Err(r.reason),
            };
            append_reports(&mut out, &reports, &state, &faces)?;
        }
    }
    Ok(out)
}
