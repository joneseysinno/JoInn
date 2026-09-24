//! Lock round-trip: write a two-item score under target/, read back; refuse off-by-one.

use std::fs;

use super::check_ok::check_ok;
use super::honest_crossing::honest_crossing;
use crate::fns::grade_opposed::grade_opposed;
use crate::fns::mutate::Mutation;
use crate::fns::parse_lock_scores::parse_lock_scores;
use crate::fns::scores_match::scores_match;
use crate::fns::subject::Subject;
use crate::fns::workspace_root;
use crate::fns::write_lock::write_lock;
use joinn_gate::GateItem;

const UNIVERSE: &str = "corpus/phase5/universe.universe";
const PHASE: &str = "harness lock";

/// Two-item table score written under target/, compared; off-by-one names the phase.
pub(super) fn check_lock_round_trip() -> Result<(), String> {
    let items: [GateItem<Subject, Mutation>; 2] = [
        GateItem {
            name: "lock fixture a",
            check: check_ok,
            control: honest_crossing,
            control_artifact: UNIVERSE,
            opposes: Mutation::DropLink("e0"),
        },
        GateItem {
            name: "lock fixture b",
            check: check_ok,
            control: honest_crossing,
            control_artifact: UNIVERSE,
            opposes: Mutation::DropLink("e0"),
        },
    ];
    for (i, item) in items.iter().enumerate() {
        grade_opposed(i + 1, item).map_err(|e| format!("harness lock fixture: {e}"))?;
    }
    let total = items.len() as u32;
    let n = total;
    let outcomes = [(PHASE, true, n, total, false)];
    let root = workspace_root()?;
    let dir = root.join("target");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("harness_fixtures.lock");
    write_lock(&path, &outcomes)?;
    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let rows = parse_lock_scores(&text)?;
    scores_match(&outcomes, &rows)?;

    let wrong_n = n.saturating_sub(1);
    let wrong = [(PHASE, true, wrong_n, total, false)];
    write_lock(&path, &wrong)?;
    let wrong_text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let wrong_rows = parse_lock_scores(&wrong_text)?;
    match scores_match(&outcomes, &wrong_rows) {
        Err(msg) if msg.contains(PHASE) => Ok(()),
        Err(msg) => Err(format!(
            "harness lock fixture: off-by-one must name {PHASE}, got {msg}"
        )),
        Ok(()) => Err("harness lock fixture: off-by-one writer was not refused".into()),
    }
}
