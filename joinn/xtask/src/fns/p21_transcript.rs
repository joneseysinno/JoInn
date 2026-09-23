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

pub(crate) fn p21_transcript() -> bool {
    match (run_calculator_bin(), workspace_root()) {
        (Ok(got), Ok(root)) => {
            let path = root
                .join("corpus")
                .join("transcripts")
                .join("calculator.txt");
            match fs::read_to_string(&path) {
                Ok(want) => got.replace("\r\n", "\n") == want.replace("\r\n", "\n"),
                Err(_) => false,
            }
        }
        _ => false,
    }
}
