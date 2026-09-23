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

pub(crate) fn p22_register() -> bool {
    let src = "body { codex 1 genome { prim:hash as h } grants { } wires { } budget { steps 1 } lineage none }\n";
    let Ok(body) = parse_body_ok(src) else {
        return false;
    };
    let floor: BTreeSet<String> = joinn_prim::floor_register()
        .iter()
        .map(|p| p.name().to_string())
        .collect();
    let matter: BTreeSet<String> = joinn_prim::floor_register()
        .iter()
        .filter(|p| p.register() == joinn_prim::Register::Matter)
        .map(|p| p.name().to_string())
        .collect();
    matches!(
        joinn_gate::check::v33::check(
            &body,
            "format@ℤ",
            &floor,
            &matter,
            joinn_frame::Hash::from_bytes([0; 32]),
            &[],
            &BTreeMap::new(),
            &BTreeMap::new(),
        ),
        Verdict::Refused(r) if r.reason.contains("physics") || r.reason.contains("register")
    )
}
