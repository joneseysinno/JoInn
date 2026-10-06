//! `cargo xtask gate all`: every phase gate, the lock, and the time per phase.

use std::fs;
use std::io::{self, Write};
use std::time::Instant;

use super::gate_table::{GateTable, phase_gates};
use super::{
    corpus_verify, harness_fixtures, parse_lock_scores, scores_match, workspace_root, write_lock,
};

/// Every phase label the lock and gate tables print. One place only.
pub(crate) const PHASE_LABELS: &[&str] = &[
    "phase 0",
    "phase 1",
    "phase 2",
    "phase 2.1",
    "phase 2.2",
    "phase 3",
    "phase 4",
    "phase 5",
    "phase 5.1",
    "phase 5.2",
    "phase 6",
    "phase 7",
    "phase 7.2",
];

/// Phase 0 (harness fixtures and the corpus), then each phase gate in lock
/// order. After the lock, one `<label> ms <n> (information)` line per phase and
/// the total. No time reaches a decision (rule 4).
// Rule 4: xtask MAY measure wall time.
#[allow(clippy::disallowed_methods)]
pub(crate) fn gate_all() -> Result<(), String> {
    let t0 = Instant::now();
    harness_fixtures()?;
    let p0 = corpus_verify().is_ok();
    if !p0 {
        corpus_verify()?;
    }
    let mut times = vec![(PHASE_LABELS[0], t0.elapsed().as_millis())];
    let mut results = Vec::new();
    for gate in phase_gates() {
        let t = Instant::now();
        let legacy = matches!(gate.table, GateTable::Legacy(_));
        results.push((gate.label, legacy, (gate.run)(gate.label)));
        times.push((gate.label, t.elapsed().as_millis()));
    }
    let mut outcomes = vec![(PHASE_LABELS[0], p0, 1, 1, false)];
    for (label, legacy, result) in results {
        let (ok, n, total) = match result {
            Ok((n, total)) => (n == total, n, total),
            Err(e) => {
                let _ = writeln!(io::stderr(), "{label}: {e}");
                (false, 0, 0)
            }
        };
        outcomes.push((label, ok, n, total, legacy));
    }
    let lock = workspace_root()?.join("gates.lock");
    write_lock(&lock, &outcomes)?;
    let text = fs::read_to_string(&lock).map_err(|e| e.to_string())?;
    let rows = parse_lock_scores(&text)?;
    scores_match(&outcomes, &rows)?;
    println!("{}", text.trim());
    for (label, ms) in &times {
        println!("{label} ms {ms} (information)");
    }
    println!("gate all wall milliseconds: {}", t0.elapsed().as_millis());
    if outcomes.iter().any(|(_, ok, _, _, _)| !ok) {
        return Err("gate all: a phase failed".into());
    }
    Ok(())
}
