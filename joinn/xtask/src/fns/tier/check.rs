//! `cargo xtask check [--gates 6,7,7.2]`: the per-commit tier (Run 7.3–9 §3).

use super::Step;
use super::run_step::run_step;
use super::test_counts::test_counts;
use crate::fns::{say, workspace_root};
use std::path::PathBuf;

/// Run fmt, clippy, the suite, the scans, zoom, links and grow (and the named gates), every
/// one even after a failure, and print the commit's Check lines from their
/// output. Any failure names the steps and the failing tests and exits 1.
pub(crate) fn check(args: Vec<String>) -> Result<(), String> {
    let gates: Vec<String> = match args.as_slice() {
        [] => Vec::new(),
        [flag, list] if flag == "--gates" => list.split(',').map(str::to_owned).collect(),
        _ => return Err("usage: cargo xtask check [--gates 6,7,7.2]".into()),
    };
    #[allow(clippy::disallowed_methods)] // wall time is information (rule 4)
    let start = std::time::Instant::now();
    let root = workspace_root()?;
    let cargo = std::env::var_os("CARGO").map_or_else(|| PathBuf::from("cargo"), PathBuf::from);
    let xtask = std::env::current_exe().map_err(|e| format!("check: own path: {e}"))?;
    let cargo_step = |args: &[&str]| run_step(&cargo, args, &root, &[]);
    let xtask_step = |args: &[&str]| run_step(&xtask, args, &root, &[]);

    let fmt = cargo_step(&["fmt", "--all", "--", "--check"]);
    let clippy = cargo_step(&[
        "clippy",
        "--workspace",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ]);
    let test = cargo_step(&["test", "--workspace", "--no-fail-fast"]);
    let scans: Vec<(&str, Step)> = ["vocab", "modules", "layers"]
        .into_iter()
        .map(|scan| (scan, xtask_step(&[scan])))
        .collect();
    let zoom = xtask_step(&["zoom"]);
    let links = xtask_step(&["links"]);
    let grow = xtask_step(&["grow"]);
    let gate_steps: Vec<(&str, Step)> = gates
        .iter()
        .map(|g| (g.as_str(), xtask_step(&["gate", g])))
        .collect();

    let exit = |step: &Step| match step.code {
        Some(code) => format!("exit {code}"),
        None => "exit without a code".to_owned(),
    };
    let (passed, failed, failing) = test_counts(&test.out);
    say(&format!(
        "Suite:      cargo test --workspace --no-fail-fast → {passed} passed, {failed} failed"
    ));
    say(&format!("Fmt/Clippy: {} · {}", exit(&fmt), exit(&clippy)));
    let scan_line: Vec<String> = scans
        .iter()
        .map(|(scan, step)| format!("{scan} → {}", step.last_line()))
        .collect();
    say(&format!("Scans:      {}", scan_line.join(" · ")));
    say(&format!(
        "Zoom:       cargo xtask zoom → {}",
        zoom.last_line()
    ));
    say(&format!(
        "Links:      cargo xtask links → {}",
        links.last_line()
    ));
    say(&format!(
        "Grow:       cargo xtask grow → {}",
        grow.last_line()
    ));
    if !gate_steps.is_empty() {
        let gate_line: Vec<String> = gate_steps
            .iter()
            .map(|(g, step)| format!("gate {g} → {}", step.last_line()))
            .collect();
        say(&format!("Gates:      {}", gate_line.join(" · ")));
    }

    let mut named: Vec<(String, &Step)> = vec![
        ("fmt".to_owned(), &fmt),
        ("clippy".to_owned(), &clippy),
        ("test".to_owned(), &test),
    ];
    named.extend(scans.iter().map(|(scan, step)| ((*scan).to_owned(), step)));
    named.push(("zoom".to_owned(), &zoom));
    named.push(("links".to_owned(), &links));
    named.push(("grow".to_owned(), &grow));
    named.extend(
        gate_steps
            .iter()
            .map(|(g, step)| (format!("gate {g}"), step)),
    );
    let failures: Vec<&(String, &Step)> = named.iter().filter(|(_, step)| !step.ok).collect();
    let ms = start.elapsed().as_millis();
    if failures.is_empty() {
        say(&format!("check: ok ({ms} ms, information)"));
        return Ok(());
    }
    for name in &failing {
        say(&format!("failing test: {name}"));
    }
    for (name, step) in &failures {
        if name != "test" {
            say(&format!("{name}: {}", step.last_line()));
        }
    }
    let names: Vec<&str> = failures.iter().map(|(name, _)| name.as_str()).collect();
    Err(format!(
        "check: failed: {} ({ms} ms, information)",
        names.join(", ")
    ))
}
