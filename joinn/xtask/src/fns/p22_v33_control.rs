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

pub(crate) fn p22_v33_control(_art: &()) -> bool {
    let Ok((bodies, cells, seals)) = load_seals() else {
        return true;
    };
    let Some(s) = seals.iter().find(|s| s.sealed.0 == "add@ℤ") else {
        return true;
    };
    let Some(body) = bodies.get(&s.reference.hash) else {
        return true;
    };
    let floor: BTreeSet<String> = joinn_prim::floor_register()
        .iter()
        .map(|p| p.name().to_string())
        .collect();
    matches!(
        joinn_prim::check_v33_full(body, &s.sealed, &floor, s.cell, &seals, &cells, &bodies),
        Verdict::Refused(_)
    )
}
