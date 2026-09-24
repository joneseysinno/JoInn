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

pub(crate) fn p21_v33_control(_art: &()) -> bool {
    let mut floor = BTreeSet::new();
    floor.insert("eq".into());
    floor.insert("case".into());
    let src = "body { codex 1 genome { prim:eq as e prim:case as c } grants { } wires { } budget { steps 1 } lineage none }\n";
    let Ok(body) = parse_body_ok(src) else {
        return true;
    };
    matches!(
        joinn_prim::check_v33(&body, &NativeId("add@ℤ".into()), &floor),
        Verdict::Refused(_)
    )
}
