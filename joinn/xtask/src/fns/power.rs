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

pub(crate) fn power() -> Result<(), String> {
    let (n12, total12, lines) = demos::power(
        Budget {
            seed: 1,
            law_samples: 48,
            testimony_samples: 8,
            size: 16,
        },
        joinn_prim::natives_with_mutants(),
    );
    for line in lines {
        println!("{line}");
    }
    let extra = power_phase2()?;
    let mut n = n12;
    let total = total12 + extra.len();
    for (i, (ok, name, check, control_ok)) in extra.iter().enumerate() {
        if *ok && *control_ok {
            n += 1;
            println!("  {:>2}  {name}  refused by {check}", i + 13);
        } else if !*ok {
            println!("  {:>2}  {name}  SURVIVED", i + 13);
        } else {
            println!(
                "  {:>2}  {name}  refused but negative control also refused",
                i + 13
            );
        }
    }
    println!("gate power: {n}/{total}");
    if n < total {
        Err(format!("gate power below 100% ({n}/{total})"))
    } else {
        Ok(())
    }
}
