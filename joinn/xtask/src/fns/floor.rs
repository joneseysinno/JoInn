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

pub(crate) fn floor() -> Result<(), String> {
    match joinn_prim::check_pairing() {
        Verdict::Ok(pairs) => {
            let floor = joinn_prim::floor_register();
            for (a, b) in &pairs {
                let reg = floor
                    .iter()
                    .find(|p| p.name() == a.as_str() || p.name() == b.as_str())
                    .map(|p| match p.register() {
                        joinn_prim::Register::Matter => "matter",
                        joinn_prim::Register::Space => "space",
                        joinn_prim::Register::Physics => "physics",
                    })
                    .unwrap_or("?");
                println!("{a} ↔ {b}  {reg}");
            }
            let n = pairs.len() * 2;
            if !n.is_multiple_of(2) {
                return Err(format!("floor count {n} is odd; an opposition is missing"));
            }
            println!("floor: {n} members, paired");
            Ok(())
        }
        Verdict::Refused(r) => Err(r.reason),
    }
}
