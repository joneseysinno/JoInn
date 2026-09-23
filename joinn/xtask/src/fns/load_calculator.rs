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

pub(crate) fn load_calculator() -> Result<(joinn_dna::Body, BTreeMap<Hash, joinn_dna::Cell>), String>
{
    let root = workspace_root()?;
    let frames = FrameRegistry::phase1();
    let src = fs::read_to_string(root.join("corpus").join("phase2").join("calculator.body"))
        .map_err(|e| e.to_string())?;
    let body = match parse_body(&src, &frames) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let mut cells = BTreeMap::new();
    for name in ["sum", "format", "cli_input"] {
        let path = root
            .join("corpus")
            .join("phase0")
            .join(format!("{name}.cell"));
        let src = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let cell = match parse_cell(&src, &frames) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => return Err(r.reason),
        };
        cells.insert(hash(&cell.coding), cell);
    }
    Ok((body, cells))
}
