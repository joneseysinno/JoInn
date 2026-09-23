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

pub(crate) fn write_lock(outcomes: &[(&str, bool, u32, u32)]) -> Result<(), String> {
    let lock = workspace_root()?.join("gates.lock");
    let mut body = String::from(
        "# JoInn gates.lock — recorded passed gates. Never edit to make a check pass.\n",
    );
    for (name, ok, n, total) in outcomes {
        if *name == "phase 0" {
            body.push_str("phase 0: ");
            body.push_str(if *ok { "pass" } else { "fail" });
            body.push('\n');
        } else {
            body.push_str(&format!("{name}: {n}/{total}\n"));
        }
    }
    fs::write(lock, body).map_err(|e| e.to_string())
}
