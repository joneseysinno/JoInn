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

pub(crate) fn p22_lookup() -> bool {
    let Ok((_, cells, seals)) = load_seals() else {
        return false;
    };
    for s in &seals {
        match joinn_prim::find_cell_for_native(&cells, &s.sealed) {
            Verdict::Ok(h) if h == s.cell => {}
            _ => return false,
        }
    }
    let Some(mul) = seals.iter().find(|s| s.sealed.0 == "mul@ℤ") else {
        return false;
    };
    mul.cell.to_hex().starts_with("12b6")
}
