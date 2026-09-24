//! Run the phase 5 universe: calculator, then the unlinked factor.

use crate::load::{find_corpus, gather_bodies, load_cells, value_in_frame};
use crate::present::{fill_present, prompt};
use joinn_dna::Body;
use joinn_frame::Verdict;
use joinn_host::{describe, describe_refusal};
use joinn_link::{
    bind_bodies, grant, parse_universe, Address, Mark, UniverseReport, UniverseState,
};
use joinn_live::BodyState;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;

use super::in_ports::in_ports;
use super::read_line::read_line;

/// Prompt unlinked in-ports in alias order, then present what fired.
pub(in crate::session) fn run_universe(
    lines: &mut impl Iterator<Item = io::Result<String>>,
) -> Result<(String, Vec<u8>), String> {
    let corpus = find_corpus()?;
    let cells = load_cells(&corpus)?;
    let by_hash = gather_bodies(&corpus, &cells)?;
    let src = fs::read_to_string(corpus.join("phase5").join("universe.universe"))
        .map_err(|e| e.to_string())?;
    let universe = match parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let bound = match bind_bodies(&universe, &by_hash) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let faces: BTreeMap<String, Body> = bound
        .iter()
        .map(|(alias, (body, _))| (alias.clone(), body.clone()))
        .collect();
    let mut state = match UniverseState::new(&universe, bound, joinn_prim::sealed_natives()) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(r.reason),
    };
    if universe.coding.links.iter().any(|link| link.id == "e0") {
        match grant(state.link_runtime(), &universe, "e0", "units") {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Err(r.reason),
        }
    }
    let mut fed = BTreeSet::new();
    for link in &universe.coding.links {
        for member in &link.members {
            if member.mark == Mark::Head {
                fed.insert((member.body.clone(), member.instance.clone(), member.port));
            }
        }
    }
    let mut groups: Vec<(String, Vec<joinn_link::BoundaryPort>)> = Vec::new();
    for (alias, body) in &faces {
        let ports = in_ports(body, &cells)
            .into_iter()
            .filter(|port| {
                !fed.contains(&(alias.clone(), port.address.instance.clone(), port.address.port))
            })
            .collect::<Vec<_>>();
        if !ports.is_empty() {
            groups.push((alias.clone(), ports));
        }
    }
    let mut out = String::new();
    let natives = joinn_prim::sealed_natives();
    for (index, (alias, ports)) in groups.iter().enumerate() {
        let Some(body) = faces.get(alias) else {
            continue;
        };
        let mut values = Vec::new();
        if index == 0 {
            let first = read_line(lines)?;
            out.push_str(prompt(body, &ports[0].address.instance, "> "));
            out.push_str(&first);
            out.push('\n');
            let mut probe = match BodyState::new(body.clone(), cells.clone(), natives.clone(), 1) {
                Verdict::Ok(s) => s,
                Verdict::Refused(r) => return Err(r.reason),
            };
            match probe.inject(
                &ports[0].address.instance,
                ports[0].address.port,
                value_in_frame(&ports[0].frame, &first)?,
                0,
            ) {
                Verdict::Ok(()) => {}
                Verdict::Refused(r) => return Err(r.reason),
            }
            let refused = match probe.run() {
                Verdict::Refused(r) => {
                    let d = describe_refusal(&probe, &ports[0].address.instance, &r.reason);
                    out.push_str("   ");
                    out.push_str(&d.label);
                    out.push('\n');
                    true
                }
                Verdict::Ok(_) => false,
            };
            if refused {
                for port in ports {
                    let line = read_line(lines)?;
                    out.push_str(prompt(body, &port.address.instance, "> "));
                    out.push_str(&line);
                    out.push('\n');
                    values.push(line);
                }
            } else {
                values.push(first);
                for port in ports.iter().skip(1) {
                    let line = read_line(lines)?;
                    out.push_str(prompt(body, &port.address.instance, "> "));
                    out.push_str(&line);
                    out.push('\n');
                    values.push(line);
                }
            }
        } else {
            for port in ports {
                let line = read_line(lines)?;
                out.push_str(prompt(body, &port.address.instance, "> "));
                out.push_str(&line);
                out.push('\n');
                values.push(line);
            }
        }
        for (i, (port, line)) in ports.iter().zip(values.iter()).enumerate() {
            match state.inject(
                alias,
                &Address {
                    instance: port.address.instance.clone(),
                    port: port.address.port,
                },
                value_in_frame(&port.frame, line)?,
                i as u64,
            ) {
                Verdict::Ok(()) => {}
                Verdict::Refused(r) => return Err(r.reason),
            }
        }
        let reports = match state.run() {
            Verdict::Ok(rs) => rs,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let mut seen = BTreeSet::new();
        for report in &reports {
            let UniverseReport::Fired { body: fired, instance } = report else {
                continue;
            };
            if !seen.insert((fired.clone(), instance.clone())) {
                continue;
            }
            let Some(live) = state.body(fired) else {
                continue;
            };
            let d = match describe(live, instance) {
                Verdict::Ok(d) => d,
                Verdict::Refused(r) => return Err(r.reason),
            };
            let Some(face) = faces.get(fired) else {
                continue;
            };
            if let Some(line) = fill_present(face, &d) {
                out.push_str(&line);
                out.push('\n');
            }
        }
    }
    Ok((out, Vec::new()))
}

#[cfg(test)]
mod tests {
    use super::run_universe;

    #[test]
    fn universe_transcript_matches_golden() {
        let lines = ["two", "2", "3", "12"].map(|s| Ok(s.to_owned()));
        let (out, _) = match run_universe(&mut lines.into_iter()) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        };
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("corpus")
            .join("transcripts");
        let want = std::fs::read(&root.join("universe.txt")).unwrap_or_else(|e| panic!("{e}"));
        let calc = std::fs::read(&root.join("calculator.txt")).unwrap_or_else(|e| panic!("{e}"));
        let got = out.into_bytes();
        assert_eq!(got, want, "{}", String::from_utf8_lossy(&got));
        assert_eq!(&got[..calc.len()], calc.as_slice());
    }
}
