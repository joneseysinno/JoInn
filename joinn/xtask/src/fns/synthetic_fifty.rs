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

pub(crate) fn synthetic_fifty() -> Result<u64, String> {
    let sum = sum_cell();
    let sh = hash(&sum.coding);
    let mut names = Vec::new();
    for i in 0..50 {
        names.push(format!("s{i}"));
    }
    let list = names.join(", ");
    let src = format!(
        "body {{ codex 1 genome {{ cell:{} as {list} }} grants {{ }} wires {{ }} budget {{ steps 100000 }} lineage none }}\n",
        sh.to_hex()
    );
    let body = match parse_body(&src, &FrameRegistry::phase1()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let mut cells = BTreeMap::new();
    cells.insert(sh, sum);
    let mut state = match BodyState::new(body, cells, joinn_prim::sealed_natives(), 1) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(r.reason),
    };
    for i in 0..50 {
        let n = format!("s{i}");
        match state.inject(&n, 0, int_val(1)?, 0) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Err(r.reason),
        }
        match state.inject(&n, 1, int_val(1)?, 0) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Err(r.reason),
        }
    }
    match state.run() {
        Verdict::Ok(_) => Ok(state.steps()),
        Verdict::Refused(r) => Err(r.reason),
    }
}
