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

pub(crate) fn refuse_false_law(root: &Path, frames: &FrameRegistry) -> Result<(), String> {
    let path = root
        .join("corpus")
        .join("phase22")
        .join("counterfeit")
        .join("false_law.cell");
    let src = fs::read_to_string(&path).map_err(|e| format!("{path:?}: {e}"))?;
    let cell = match parse_cell(&src, frames) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => return Err(format!("false_law.cell parse: {}", r.reason)),
    };
    let gate = Gate::new(Budget::default(), joinn_prim::sealed_natives());
    let mut refused = false;
    for allele in &cell.alleles {
        if matches!(gate.admit_allele(&cell, allele), Verdict::Refused(_)) {
            refused = true;
        }
    }
    if !refused {
        return Err("false_law.cell was admitted; the control must refuse".into());
    }
    println!("corpus verify: refused false_law.cell by name");
    Ok(())
}
