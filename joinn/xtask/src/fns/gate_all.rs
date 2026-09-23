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

// Rule 4: xtask MAY measure wall time.
#[allow(clippy::disallowed_methods)]
pub(crate) fn gate_all() -> Result<(), String> {
    let t0 = Instant::now();
    let p0 = corpus_verify().is_ok();
    if !p0 {
        corpus_verify()?;
    }
    let p1 = gate_one();
    let p2 = gate_two();
    let p21 = gate_two_one();
    let p22 = gate_two_two();
    let p3 = gate_three();
    let p5 = gate_five();
    let p51 = gate_five_one();
    let score = |label: &str, result: Result<(u32, u32), String>| match result {
        Ok((n, total)) => (n == total, n, total),
        Err(e) => {
            let _ = writeln!(io::stderr(), "{label}: {e}");
            (false, 0, 0)
        }
    };
    let (p1_ok, p1_n, p1_t) = score("phase 1", p1);
    let (p2_ok, p2_n, p2_t) = score("phase 2", p2);
    let (p21_ok, p21_n, p21_t) = score("phase 2.1", p21);
    let (p22_ok, p22_n, p22_t) = score("phase 2.2", p22);
    let (p3_ok, p3_n, p3_t) = score("phase 3", p3);
    let (p5_ok, p5_n, p5_t) = score("phase 5", p5);
    let phase_51 = format!("phase {}.{}", 5, 1);
    let (p51_ok, p51_n, p51_t) = score(&phase_51, p51);
    let outcomes = [
        ("phase 0", p0, 1, 1),
        ("phase 1", p1_ok, p1_n, p1_t),
        ("phase 2", p2_ok, p2_n, p2_t),
        ("phase 2.1", p21_ok, p21_n, p21_t),
        ("phase 2.2", p22_ok, p22_n, p22_t),
        ("phase 3", p3_ok, p3_n, p3_t),
        ("phase 5", p5_ok, p5_n, p5_t),
        (phase_51.as_str(), p51_ok, p51_n, p51_t),
    ];
    write_lock(&outcomes)?;
    let lock = workspace_root()?.join("gates.lock");
    let text = fs::read_to_string(&lock).map_err(|e| e.to_string())?;
    let rows = parse_lock_scores(&text)?;
    scores_match(&outcomes, &rows)?;
    println!("{}", text.trim());
    println!("gate all wall milliseconds: {}", t0.elapsed().as_millis());
    if !p0 || !p1_ok || !p2_ok || !p21_ok || !p22_ok || !p3_ok || !p5_ok || !p51_ok {
        return Err("gate all: a phase failed".into());
    }
    Ok(())
}
