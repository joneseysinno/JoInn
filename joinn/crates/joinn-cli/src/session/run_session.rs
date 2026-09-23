//! Load a body, prompt each membrane in-port, present fires and a refusal.

use crate::cli_host::CliHost;
use crate::load::{environment_signals, find_corpus, load_body, load_cells, value_in_frame};
use crate::present::prompt;
use joinn_frame::Verdict;
use joinn_host::{Host, check_signals, describe, describe_refusal};
use joinn_live::BodyState;
use std::collections::BTreeSet;
use std::io;

use super::in_ports::in_ports;
use super::read_line::read_line;

/// Run `name`. The first line is aimed at the first in-port. A membrane
/// refusal is presented, and then a line is read per in-port. An accepted
/// first line is that port's value, and the remaining ports are read after it.
pub(in crate::session) fn run_session(
    name: &str,
    lines: &mut impl Iterator<Item = io::Result<String>>,
) -> Result<(String, Vec<u8>), String> {
    let corpus = find_corpus()?;
    let body = load_body(&corpus, name)?;
    let cells = load_cells(&corpus)?;
    let natives = joinn_prim::sealed_natives();
    let ports = in_ports(&body, &cells);
    if ports.is_empty() {
        return Err(format!(
            "body `{name}` exposes no membrane in-port; acceptance is a body that exposes an in-port"
        ));
    }

    let mut host = CliHost::new(body.clone(), cells.clone(), environment_signals()?);
    match check_signals(&body, host.signals()) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Err(r.reason),
    }

    let first = read_line(lines)?;
    let lead = prompt(&body, &ports[0].address.instance, "> ");
    host.out.push_str(lead);
    host.out.push_str(&first);
    host.out.push('\n');

    let mut probe = match BodyState::new(body.clone(), cells.clone(), natives.clone(), 1) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(r.reason),
    };
    match probe.inject(&ports[0].address.instance, ports[0].address.port, value_in_frame(&ports[0].frame, &first)?, 0) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Err(r.reason),
    }
    let probe_refused = match probe.run() {
        Verdict::Refused(r) => {
            let d = describe_refusal(&probe, &ports[0].address.instance, &r.reason);
            match host.present(&d) {
                Verdict::Ok(()) => true,
                Verdict::Refused(e) => return Err(e.reason),
            }
        }
        Verdict::Ok(_) => false,
    };

    let mut values = Vec::new();
    if probe_refused {
        for port in &ports {
            let line = read_line(lines)?;
            let text = prompt(&body, &port.address.instance, "> ");
            host.out.push_str(text);
            host.out.push_str(&line);
            host.out.push('\n');
            values.push(line);
        }
    } else {
        values.push(first);
        for port in ports.iter().skip(1) {
            let line = read_line(lines)?;
            let text = prompt(&body, &port.address.instance, "> ");
            host.out.push_str(text);
            host.out.push_str(&line);
            host.out.push('\n');
            values.push(line);
        }
    }

    let mut state = match BodyState::new(body.clone(), cells.clone(), natives, 1) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(r.reason),
    };
    for (i, (port, line)) in ports.iter().zip(values.iter()).enumerate() {
        host.pending = Some(port.address.clone());
        match host.intend(line.clone()) {
            Verdict::Ok(Some(_)) => {}
            Verdict::Ok(None) => {
                return Err(format!(
                    "no intent at {}; acceptance is a line for that in-port",
                    port.address.printed()
                ));
            }
            Verdict::Refused(r) => return Err(r.reason),
        }
        match state.inject(&port.address.instance, port.address.port, value_in_frame(&port.frame, line)?, i as u64) {
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
        let Some(fired) = &report.fired else {
            continue;
        };
        if !seen.insert(fired.clone()) {
            continue;
        }
        match describe(&state, fired) {
            Verdict::Ok(d) => match host.present(&d) {
                Verdict::Ok(()) => {}
                Verdict::Refused(e) => return Err(e.reason),
            },
            Verdict::Refused(r) => return Err(r.reason),
        }
    }

    let mut trace = Vec::new();
    for report in &reports {
        trace.extend(report.to_bytes());
    }
    Ok((host.out, trace))
}

#[cfg(test)]
mod tests {
    use super::run_session;

    #[test]
    fn transcript_matches_golden() {
        let lines = ["two", "2", "3"].map(|s| Ok(s.to_owned()));
        let (out, _) = match run_session("calculator", &mut lines.into_iter()) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        };
        let want = match std::fs::read_to_string(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("..")
                .join("corpus")
                .join("transcripts")
                .join("calculator.txt"),
        ) {
            Ok(s) => s,
            Err(e) => panic!("{e}"),
        };
        assert_eq!(out, want);
    }

    #[test]
    fn units_runs_standalone() {
        let lines = ["7", "12"].map(|s| Ok(s.to_owned()));
        let (out, _) = match run_session("units", &mut lines.into_iter()) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        };
        assert!(
            out.contains("7 ft = 84 in"),
            "expected 7 ft = 84 in, got {out}"
        );
    }
}
