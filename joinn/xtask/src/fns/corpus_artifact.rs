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

pub(crate) fn corpus_artifact(root: &Path, name: &str) -> PathBuf {
    if name.ends_with(".desc") {
        return root.join("corpus").join("descriptions").join(name);
    }
    if name.ends_with(".universe") {
        let phase5 = root.join("corpus").join("phase5").join(name);
        if phase5.exists() {
            return phase5;
        }
        return root.join("corpus").join("phase5").join("controls").join(name);
    }
    for dir in [
        "phase5",
        "phase5/controls",
        "phase51/controls",
        "phase3",
        "phase22",
        "phase21",
        "phase2",
        "phase2/variants",
        "phase0",
    ] {
        for ext in ["cell", "body"] {
            let p = root.join("corpus").join(dir).join(format!("{name}.{ext}"));
            if p.exists() {
                return p;
            }
        }
    }
    root.join("corpus")
        .join("phase2")
        .join(format!("{name}.body"))
}
