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

pub(crate) fn p22_roundtrip_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok((bodies, cells, mut seals)) = load_seals() else {
        return true;
    };
    for s in &mut seals {
        if s.sealed.0 == "parse@Text" {
            s.one_way = false;
        }
    }
    let dna = LiveDna::new(joinn_prim::engine_natives());
    matches!(
        joinn_prim::agree(&dna, &bodies, &cells, &seals, 1, 4),
        Verdict::Ok(_)
    )
}
