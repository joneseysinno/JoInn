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

pub(crate) fn p21_v33() -> bool {
    let floor_names: BTreeSet<String> = joinn_prim::floor_register()
        .iter()
        .map(|p| p.name().to_string())
        .collect();
    let mutant14 = joinn_prim::mutant_sealed_as_reference_body();
    matches!(
        joinn_prim::check_v33(&mutant14, &NativeId("add@ℤ".into()), &floor_names),
        Verdict::Refused(r) if r.reason.contains("check::v33") && r.check == joinn_frame::CheckId::V33
    )
}
