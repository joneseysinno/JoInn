//! Auto-split leaf.
#![allow(unused_imports)]

use joinn_dna::{
    AlleleBody, NativeId, hash, parse_body, parse_cell, print_body, print_coding, sum_cell,
};
use joinn_frame::{Frame, FrameRegistry, Hash, IntFrame, Term, TextFrame, Value, Verdict};
use joinn_gate::{Budget, Gate, GateItem, demos, run_opposed};
use joinn_live::{BodyState, LiveDna};
use joinn_prim::{BodyRef, DnaFire, Drive, Seal};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::Instant;

use super::*;

/// Every phase label the lock and gate tables print. One place only.
pub(crate) const PHASE_LABELS: &[&str] = &[
    "phase 0",
    "phase 1",
    "phase 2",
    "phase 2.1",
    "phase 2.2",
    "phase 3",
    "phase 5",
    "phase 5.1",
    "phase 5.2",
];

// Rule 4: xtask MAY measure wall time.
#[allow(clippy::disallowed_methods)]
pub(crate) fn gate_all() -> Result<(), String> {
    let t0 = Instant::now();
    harness_fixtures()?;
    let p0 = corpus_verify().is_ok();
    if !p0 {
        corpus_verify()?;
    }
    let p1 = gate_one(PHASE_LABELS[1]);
    let p2 = gate_two(PHASE_LABELS[2]);
    let p21 = gate_two_one(PHASE_LABELS[3]);
    let p22 = gate_two_two(PHASE_LABELS[4]);
    let p3 = gate_three(PHASE_LABELS[5]);
    let p5 = gate_five(PHASE_LABELS[6]);
    let p51 = gate_five_one(PHASE_LABELS[7]);
    let p52 = gate_five_two(PHASE_LABELS[8]);
    let score = |label: &str, result: Result<(u32, u32), String>| match result {
        Ok((n, total)) => (n == total, n, total),
        Err(e) => {
            let _ = writeln!(io::stderr(), "{label}: {e}");
            (false, 0, 0)
        }
    };
    let (p1_ok, p1_n, p1_t) = score(PHASE_LABELS[1], p1);
    let (p2_ok, p2_n, p2_t) = score(PHASE_LABELS[2], p2);
    let (p21_ok, p21_n, p21_t) = score(PHASE_LABELS[3], p21);
    let (p22_ok, p22_n, p22_t) = score(PHASE_LABELS[4], p22);
    let (p3_ok, p3_n, p3_t) = score(PHASE_LABELS[5], p3);
    let (p5_ok, p5_n, p5_t) = score(PHASE_LABELS[6], p5);
    let (p51_ok, p51_n, p51_t) = score(PHASE_LABELS[7], p51);
    let (p52_ok, p52_n, p52_t) = score(PHASE_LABELS[8], p52);
    let outcomes = [
        (PHASE_LABELS[0], p0, 1, 1, false),
        (PHASE_LABELS[1], p1_ok, p1_n, p1_t, true),
        (PHASE_LABELS[2], p2_ok, p2_n, p2_t, true),
        (PHASE_LABELS[3], p21_ok, p21_n, p21_t, true),
        (PHASE_LABELS[4], p22_ok, p22_n, p22_t, true),
        (PHASE_LABELS[5], p3_ok, p3_n, p3_t, true),
        (PHASE_LABELS[6], p5_ok, p5_n, p5_t, false),
        (PHASE_LABELS[7], p51_ok, p51_n, p51_t, false),
        (PHASE_LABELS[8], p52_ok, p52_n, p52_t, false),
    ];
    let lock = workspace_root()?.join("gates.lock");
    write_lock(&lock, &outcomes)?;
    let text = fs::read_to_string(&lock).map_err(|e| e.to_string())?;
    let rows = parse_lock_scores(&text)?;
    scores_match(&outcomes, &rows)?;
    println!("{}", text.trim());
    println!("gate all wall milliseconds: {}", t0.elapsed().as_millis());
    if !p0 || !p1_ok || !p2_ok || !p21_ok || !p22_ok || !p3_ok || !p5_ok || !p51_ok || !p52_ok {
        return Err("gate all: a phase failed".into());
    }
    Ok(())
}
