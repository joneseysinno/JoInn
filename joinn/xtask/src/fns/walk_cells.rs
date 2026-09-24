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

pub(crate) fn walk_cells(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    if !dir.exists() {
        return Ok(());
    }
    let entries = fs::read_dir(dir).map_err(|e| e.to_string())?;
    let mut paths: Vec<_> = entries
        .map(|e| e.map_err(|err| err.to_string()).map(|ent| ent.path()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    for path in paths {
        if path.is_dir() {
            if path.file_name().is_some_and(|n| n == "counterfeit") {
                continue;
            }
            walk_cells(&path, out)?;
        } else if path.extension().is_some_and(|e| e == "cell") {
            out.push(path);
        }
    }
    Ok(())
}
