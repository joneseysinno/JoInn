//! Auto-split leaf.
#![allow(unused_imports)]

use joinn_dna::{
    AlleleBody, NativeId, hash, parse_body, parse_cell, print_body, print_coding, sum_cell,
};
use joinn_frame::{
    Frame, FrameRegistry, Hash, IntFrame, TAG_DESCRIPTION, Term, TextFrame, Value, Verdict,
    keyed_hash,
};
use joinn_gate::{Budget, Gate, GateItem, demos, run_opposed};
use joinn_link::{hash_universe, parse_universe};
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

pub(crate) fn corpus_verify() -> Result<(), String> {
    let root = workspace_root()?;
    let frames = FrameRegistry::phase1();
    let gold = fs::read_to_string(hashes_path()?).map_err(|e| e.to_string())?;
    let mut checked = 0;
    for line in gold.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let Some(name) = parts.next() else { continue };
        let Some(want) = parts.next() else {
            return Err(format!("malformed hashes.txt line: {line}"));
        };
        let path = corpus_artifact(&root, name);
        let src = fs::read_to_string(&path).map_err(|e| format!("{path:?}: {e}"))?;
        let src = src.replace("\r\n", "\n");
        let got = if path.extension().is_some_and(|e| e == "desc") {
            keyed_hash(TAG_DESCRIPTION, src.as_bytes()).to_hex()
        } else if path.extension().is_some_and(|e| e == "universe") {
            let u = match parse_universe(&src) {
                Verdict::Ok(u) => u,
                Verdict::Refused(r) => {
                    return Err(format!("{}: parse refusal: {}", path.display(), r.reason));
                }
            };
            hash_universe(&u.coding).to_hex()
        } else if path.extension().is_some_and(|e| e == "body") {
            let body = match parse_body(&src, &frames) {
                Verdict::Ok(b) => b,
                Verdict::Refused(r) => {
                    return Err(format!("{}: parse refusal: {}", path.display(), r.reason));
                }
            };
            hash(&body.coding).to_hex()
        } else {
            let cell = match parse_cell(&src, &frames) {
                joinn_frame::Verdict::Ok(c) => c,
                joinn_frame::Verdict::Refused(r) => {
                    return Err(format!("{}: parse refusal: {}", path.display(), r.reason));
                }
            };
            hash(&cell.coding).to_hex()
        };
        if got != want {
            return Err(format!(
                "hash mismatch for {name}: golden {want} computed {got}"
            ));
        }
        checked += 1;
    }
    admit_corpus_cells(&root, &frames)?;
    refuse_false_law(&root, &frames)?;
    println!("corpus verify: {checked} hash(es) match; cells admitted");
    Ok(())
}
