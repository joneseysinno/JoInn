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

pub(crate) fn p21_turn_pair() -> Result<(bool, bool), String> {
    let mut unwitnessed = {
        let path = workspace_root()?
            .join("corpus")
            .join("phase21")
            .join("mul.cell");
        let src = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        match parse_cell(&src, &FrameRegistry::phase1()) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => return Err(r.reason),
        }
    };
    unwitnessed.coding.turns = vec![joinn_dna::TurnDecl {
        out: 0,
        from: vec![1, 2],
    }];
    let gate = Gate::new(Budget::default(), joinn_prim::sealed_natives());
    let turn_refused = matches!(
        gate.admit_cell(&unwitnessed, None),
        Verdict::Refused(r) if r.reason.contains("turn")
    );
    let t1 = joinn_dna::TurnDecl {
        out: 1,
        from: vec![0, 2],
    };
    let turn1_ok = matches!(
        joinn_prim::admit_turn(
            &t1,
            &joinn_prim::alleles::AddInt,
            &joinn_prim::turns::AddIntTurn1,
            1,
            32,
        ),
        Verdict::Ok(())
    );
    Ok((turn_refused, turn1_ok))
}
