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

pub(crate) fn admit_corpus_cells(root: &Path, frames: &FrameRegistry) -> Result<(), String> {
    let mut paths = Vec::new();
    walk_cells(&root.join("corpus"), &mut paths)?;
    let mut gate = Gate::new(Budget::default(), joinn_prim::sealed_natives());
    let mut loaded = Vec::new();
    for path in paths {
        let src = fs::read_to_string(&path).map_err(|e| format!("{path:?}: {e}"))?;
        let cell = match parse_cell(&src, frames) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => {
                return Err(format!("{}: parse refusal: {}", path.display(), r.reason));
            }
        };
        if let Some(allele) = cell.alleles.first() {
            if let AlleleBody::Native(id) = &allele.body {
                if let Some(oracle) = gate.native_arc(&id.0) {
                    gate.register_cell_oracle(hash(&cell.coding), oracle);
                }
            }
        }
        loaded.push((path, cell));
    }
    for (path, cell) in &loaded {
        match gate.admit_cell(cell, None) {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) => {
                return Err(format!(
                    "{}: admit_cell refused: {}",
                    path.display(),
                    r.reason
                ));
            }
        }
        for allele in &cell.alleles {
            match gate.admit_allele(cell, allele) {
                Verdict::Ok(_) => {}
                Verdict::Refused(r) => {
                    return Err(format!(
                        "{}: admit_allele refused: {}",
                        path.display(),
                        r.reason
                    ));
                }
            }
        }
    }
    Ok(())
}
