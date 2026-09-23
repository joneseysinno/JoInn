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

pub(crate) fn budget_not_membrane() -> Result<bool, String> {
    let sum = sum_cell();
    let sh = hash(&sum.coding);
    let src = format!(
        "body {{ codex 1 genome {{ cell:{} as c }} grants {{ }} wires {{ c@2 -> c@0 }} budget {{ steps 3 }} lineage none }}\n",
        sh.to_hex()
    );
    let body = match parse_body(&src, &FrameRegistry::phase1()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let mut cells = BTreeMap::new();
    cells.insert(sh, sum);
    let mut state = match BodyState::new(body, cells, joinn_prim::sealed_natives(), 11) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let _ = state.inject("c", 0, int_val(1)?, 0);
    let _ = state.inject("c", 1, int_val(1)?, 0);
    Ok(matches!(state.run(), Verdict::Refused(r) if r.check == joinn_frame::CheckId::Budget))
}
