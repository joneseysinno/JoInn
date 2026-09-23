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

pub(crate) fn power_phase2() -> Result<Vec<(bool, &'static str, &'static str, bool)>, String> {
    let (bodies, cells, seals) = load_seals()?;
    let dna = LiveDna::new(joinn_prim::sealed_natives());
    let add_seal = seals
        .iter()
        .find(|s| s.sealed.0 == "add@ℤ")
        .ok_or_else(|| "no add seal".to_string())?;
    let wrapping = joinn_prim::wrapping_caught_by_agree(&dna, &bodies, &cells, add_seal, 1, 32);
    let floor: BTreeSet<String> = joinn_prim::floor_register()
        .iter()
        .map(|p| p.name().to_string())
        .collect();
    let mutant14 = joinn_prim::mutant_sealed_as_reference_body();
    let m14 = matches!(
        joinn_prim::check_v33(&mutant14, &NativeId("add@ℤ".into()), &floor),
        Verdict::Refused(r) if r.reason.contains("check::v33")
    );
    let control_src = "body { codex 1 genome { prim:eq as e prim:case as c } grants { } wires { } budget { steps 1 } lineage none }\n";
    let control_body = match parse_body(control_src, &FrameRegistry::phase1()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let m14_control = matches!(
        joinn_prim::check_v33(&control_body, &NativeId("add@ℤ".into()), &floor),
        Verdict::Ok(())
    );
    Ok(vec![
        (wrapping, "mutant.wrapping", "check::agree", true),
        (m14, "mutant.sealed_as_reference", "check::v33", m14_control),
        (
            join_refuse_catches_drop()?,
            "mutant.drop_second",
            "check::join",
            true,
        ),
        (
            grant_order_not_arrival()?,
            "mutant.arrival_order",
            "check::grant",
            true,
        ),
        (
            ungranted_is_refused()?,
            "mutant.ungranted",
            "check::grant",
            true,
        ),
        (
            one_delivery_path()?,
            "mutant.skip_require",
            "check::v35",
            true,
        ),
        (
            joinn_prim::admit_turn_positive_only_is_refused(),
            "mutant.turn_pos_only",
            "check::turn",
            true,
        ),
        (
            budget_not_membrane()?,
            "mutant.budget_as_membrane",
            "check::budget",
            true,
        ),
    ])
}
