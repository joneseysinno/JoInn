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

pub(crate) fn verify_phase2_bodies() -> Result<(), String> {
    let dir = workspace_root()?.join("corpus").join("phase2");
    let frames = FrameRegistry::phase1();
    let mut printed = None;
    for name in [
        "calculator.body",
        "calculator_b.body",
        "calculator_c.body",
        "calculator_d.body",
    ] {
        let src = fs::read_to_string(dir.join(name)).map_err(|e| e.to_string())?;
        let body = match parse_body(&src, &frames) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => return Err(format!("{name}: {}", r.reason)),
        };
        let text = print_body(&body.coding);
        match &printed {
            None => printed = Some(text),
            Some(prev) if prev == &text => {}
            Some(_) => return Err(format!("{name} canonical text differs")),
        }
    }
    Ok(())
}
