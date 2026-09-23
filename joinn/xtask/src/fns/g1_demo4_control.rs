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

pub(crate) fn g1_demo4_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let gate = Gate::new(Budget::default(), g1_natives());
    let parent = sum_cell();
    let mut added = parent.clone();
    added.coding.contract.ports.push(joinn_dna::PortDecl {
        position: 3,
        direction: joinn_dna::Direction::In,
        frame: joinn_frame::FrameRef::int(),
        required: false,
    });
    matches!(gate.admit_cell(&added, Some(&parent)), Verdict::Ok(_))
}
