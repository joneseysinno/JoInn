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

pub(crate) fn p21_turn_control(_art: &()) -> bool {
    let mut cell = {
        let Ok(root) = workspace_root() else {
            return true;
        };
        let path = root.join("corpus").join("phase21").join("mul.cell");
        let Ok(src) = fs::read_to_string(&path) else {
            return true;
        };
        match parse_cell(&src, &FrameRegistry::phase1()) {
            Verdict::Ok(c) => c,
            Verdict::Refused(_) => return true,
        }
    };
    cell.coding.turns = vec![joinn_dna::TurnDecl {
        out: 0,
        from: vec![1, 2],
    }];
    let gate = Gate::new(Budget::default(), joinn_prim::sealed_natives());
    matches!(gate.admit_cell(&cell, None), Verdict::Ok(_))
}
