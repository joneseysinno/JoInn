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

pub(crate) fn reference_add_steps(b: i64) -> Result<u64, String> {
    let (bodies, cells, seals) = load_seals()?;
    let dna = LiveDna::new(joinn_prim::sealed_natives());
    let add_seal = seals
        .iter()
        .find(|s| s.sealed.0 == "add@ℤ")
        .ok_or_else(|| "no add seal".to_string())?;
    let Some(body) = bodies.get(&add_seal.reference.hash) else {
        return Err("add reference body missing".into());
    };
    let Some(cell) = cells.get(&add_seal.cell) else {
        return Err("add cell missing".into());
    };
    let inputs = BTreeMap::from([(0, int_val(0)?), (1, int_val(b)?)]);
    match dna.fire(body, cell, &cells, &bodies, &inputs) {
        Verdict::Ok(_) => match u64::try_from(b) {
            Ok(n) => Ok(n),
            Err(_) => Err("addend must be non-negative".into()),
        },
        Verdict::Refused(r) => Err(r.reason),
    }
}
