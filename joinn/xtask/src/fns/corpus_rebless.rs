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

pub(crate) fn corpus_rebless(args: Vec<String>) -> Result<(), String> {
    if !args.iter().any(|a| a == "--i-changed-the-canonical-form") {
        return Err(
            "rebless refuses: pass --i-changed-the-canonical-form and write a finding".into(),
        );
    }
    let root = workspace_root()?;
    let frames = FrameRegistry::phase1();
    let dir = root.join("corpus").join("phase0");
    let mut lines = vec!["# JoInn corpus goldens. Never edit to make a test pass.".to_string()];
    let entries = fs::read_dir(&dir).map_err(|e| e.to_string())?;
    let mut names = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "cell") {
            if let Some(stem) = path.file_stem() {
                names.push(stem.to_string_lossy().into_owned());
            }
        }
    }
    names.sort();
    for name in names {
        let path = dir.join(format!("{name}.cell"));
        let src = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let cell = match parse_cell(&src, &frames) {
            joinn_frame::Verdict::Ok(c) => c,
            joinn_frame::Verdict::Refused(r) => return Err(r.reason),
        };
        lines.push(format!("{name} {}", hash(&cell.coding).to_hex()));
        let _ = print_coding(&cell.coding);
    }
    fs::write(hashes_path()?, lines.join("\n") + "\n").map_err(|e| e.to_string())?;
    println!("corpus rebless: wrote hashes.txt (this is the accepted rehash, not an accident)");
    println!("record the reason by hand in docs/Findings/canonical-form-changes.md");
    Ok(())
}
