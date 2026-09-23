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

pub(crate) fn load_seals() -> Result<LoadedSeals, String> {
    let root = workspace_root()?;
    let frames = FrameRegistry::phase1();
    let mut cells = BTreeMap::new();
    for (dir, name) in [
        ("phase0", "sum"),
        ("phase0", "format"),
        ("phase0", "cli_input"),
        ("phase2", "sum_turn"),
        ("phase21", "mul"),
        ("phase21", "rat_sum"),
    ] {
        let path = root.join("corpus").join(dir).join(format!("{name}.cell"));
        if !path.exists() {
            continue;
        }
        let src = fs::read_to_string(&path).map_err(|e| format!("{path:?}: {e}"))?;
        let cell = match parse_cell(&src, &frames) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => return Err(r.reason),
        };
        cells.insert(hash(&cell.coding), cell);
    }
    let mut bodies = BTreeMap::new();
    let mut seals = Vec::new();
    for spec in joinn_prim::seal_register() {
        let mut ref_path = root
            .join("corpus")
            .join("phase22")
            .join(format!("{}.body", spec.reference_file));
        if !ref_path.exists() {
            ref_path = root
                .join("corpus")
                .join("phase21")
                .join(format!("{}.body", spec.reference_file));
        }
        let src = fs::read_to_string(&ref_path).map_err(|e| format!("{ref_path:?}: {e}"))?;
        let body = match parse_body(&src, &frames) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => return Err(format!("{}: {}", spec.reference_file, r.reason)),
        };
        let bh = hash(&body.coding);
        bodies.insert(bh, body);
        let cf_path = root
            .join("corpus")
            .join("phase22")
            .join("counterfeit")
            .join(format!("{}.body", spec.counterfeit_file));
        let cf_src = fs::read_to_string(&cf_path).map_err(|e| format!("{cf_path:?}: {e}"))?;
        let cf_body = match parse_body(&cf_src, &frames) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => return Err(format!("{}: {}", spec.counterfeit_file, r.reason)),
        };
        let ch = hash(&cf_body.coding);
        bodies.insert(ch, cf_body);
        let sealed = joinn_dna::NativeId(spec.sealed.into());
        let cell_h = match joinn_prim::find_cell_for_native(&cells, &sealed) {
            Verdict::Ok(h) => h,
            Verdict::Refused(r) => return Err(r.reason),
        };
        seals.push(Seal {
            cell: cell_h,
            reference: BodyRef { hash: bh },
            counterfeit: BodyRef { hash: ch },
            sealed,
            drives: Drive::new(spec.drive_port, nz(spec.drive_bound, spec.sealed)?),
            one_way: spec.one_way,
        });
    }
    Ok((bodies, cells, seals))
}
