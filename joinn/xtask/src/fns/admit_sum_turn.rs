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

pub(crate) fn admit_sum_turn() -> Result<(), String> {
    let root = workspace_root()?;
    let frames = FrameRegistry::phase1();
    let parent_src = fs::read_to_string(root.join("corpus").join("phase0").join("sum.cell"))
        .map_err(|e| e.to_string())?;
    let parent = match parse_cell(&parent_src, &frames) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let parent_hash = hash(&parent.coding);
    if parent_hash.to_hex() != "6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39" {
        return Err(format!(
            "sum.cell moved: {} (goldens must not move)",
            parent_hash.to_hex()
        ));
    }
    let src = fs::read_to_string(root.join("corpus").join("phase2").join("sum_turn.cell"))
        .map_err(|e| e.to_string())?;
    let proposed = match parse_cell(&src, &frames) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let want = parent_hash;
    admit_lineage(&proposed, &parent)?;
    match joinn_prim::admit_add_turn0() {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Err(r.reason),
    }
    println!(
        "evolution: sum_turn admitted against {} under check 4",
        want.short_hex()
    );
    Ok(())
}
