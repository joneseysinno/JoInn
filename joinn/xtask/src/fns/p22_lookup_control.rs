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

pub(crate) fn p22_lookup_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let empty = BTreeMap::new();
    let none_ok = matches!(
        joinn_prim::find_cell_for_native(&empty, &NativeId("add@ℤ".into())),
        Verdict::Ok(_)
    );
    let Ok((_, cells, _)) = load_seals() else {
        return true;
    };
    let mut two = cells.clone();
    if let Some(c) = cells.values().next() {
        two.insert(joinn_frame::Hash::from_bytes([0x11; 32]), c.clone());
        two.insert(joinn_frame::Hash::from_bytes([0x22; 32]), c.clone());
    }
    let two_ok = match cells.values().next().and_then(|c| c.alleles.first()) {
        Some(a) => match &a.body {
            joinn_dna::AlleleBody::Native(id) => {
                matches!(joinn_prim::find_cell_for_native(&two, id), Verdict::Ok(_))
            }
            _ => false,
        },
        None => true,
    };
    none_ok || two_ok
}
