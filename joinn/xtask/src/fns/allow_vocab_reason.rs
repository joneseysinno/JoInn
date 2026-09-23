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

pub(crate) fn allow_vocab_reason(line: &str) -> Option<&str> {
    const MARK: &str = concat!("allow", "(vocab)");
    let idx = line.find(MARK)?;
    let rest = &line[idx + MARK.len()..];
    if let Some(r) = rest.strip_prefix(':') {
        Some(r.trim())
    } else if rest.trim_start().starts_with(':') {
        Some(rest.trim_start().trim_start_matches(':').trim())
    } else {
        Some("")
    }
}
