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

pub(crate) fn p21_trace() -> bool {
    match calculator_success_trace() {
        Ok((trace, _)) => {
            let Ok(root) = workspace_root() else {
                return false;
            };
            let path = root
                .join("corpus")
                .join("transcripts")
                .join("calculator.trace");
            match fs::read(&path) {
                Ok(recorded) => recorded == trace,
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}
