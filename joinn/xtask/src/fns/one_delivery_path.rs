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

pub(crate) fn one_delivery_path() -> Result<bool, String> {
    let (body, cells) = load_calculator()?;
    let mut state = match BodyState::new(body, cells, joinn_prim::sealed_natives(), 1) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let _ = state.inject("cli_a", 0, text_val("2")?, 0);
    let _ = state.inject("cli_b", 0, text_val("3")?, 1);
    let _ = state.run();
    let deliveries = state.delivery_count();
    let steps = state.steps();
    Ok(deliveries >= 1 && deliveries == steps.saturating_sub(1)) // allow(vocab): rust usize saturating_sub, not a turn identifier
}
