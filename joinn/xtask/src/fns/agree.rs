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

pub(crate) fn agree() -> Result<(), String> {
    let (bodies, cells, seals) = load_seals()?;
    let dna = LiveDna::new(joinn_prim::engine_natives());
    match joinn_prim::agree(&dna, &bodies, &cells, &seals, 1, joinn_prim::AGREE_SAMPLES) {
        Verdict::Ok(lines) => {
            for line in &lines {
                println!("{line}");
            }
        }
        Verdict::Refused(r) => {
            if let Some(cx) = &r.counterexample {
                println!(
                    "counter-example bindings={} expected={:?} got={:?}",
                    cx.bindings.len(),
                    cx.expected.as_ref().map(|v| v.print_term()),
                    cx.got.as_ref().map(|v| v.print_term())
                );
            }
            return Err(r.reason);
        }
    }
    match joinn_prim::agree_injected_disagreement(1, 16) {
        Verdict::Refused(r) => {
            println!("{}", r.reason);
            if r.counterexample.is_none() {
                return Err("injected disagreement lacked a counter-example".into());
            }
            println!("injected disagreement: refused as truth violation (ok)");
        }
        Verdict::Ok(s) => {
            return Err(format!("injected disagreement was admitted: {s}"));
        }
    }
    Ok(())
}
