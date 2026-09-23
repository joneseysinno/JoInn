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

pub(crate) fn p22_corpus_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok(root) = workspace_root() else {
        return true;
    };
    let path = root
        .join("corpus")
        .join("phase22")
        .join("counterfeit")
        .join("false_law.cell");
    let Ok(src) = fs::read_to_string(&path) else {
        return true;
    };
    let cell = match parse_cell(&src, &FrameRegistry::phase1()) {
        Verdict::Ok(c) => c,
        Verdict::Refused(_) => return true,
    };
    let gate = Gate::new(Budget::default(), joinn_prim::sealed_natives());
    cell.alleles
        .iter()
        .all(|a| matches!(gate.admit_allele(&cell, a), Verdict::Ok(_)))
}
