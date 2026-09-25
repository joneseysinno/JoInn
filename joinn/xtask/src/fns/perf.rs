//! Performance probes. Wall time is printed only.
//! allow(modules): six probes share one timed entry point
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
use std::path::PathBuf;
use std::process::{Command, ExitCode, Stdio};
use std::time::Instant;

use super::*;

// Rule 4: xtask MAY measure wall time.
#[allow(clippy::disallowed_methods)]
pub(crate) fn perf() -> Result<(), String> {
    let t0 = Instant::now();
    let (_, calc_steps) = calculator_success_trace()?;
    let calc_ms = t0.elapsed().as_millis();
    let t1 = Instant::now();
    let fifty_steps = synthetic_fifty()?;
    let fifty_ms = t1.elapsed().as_millis();
    let t2 = Instant::now();
    let ref_steps = reference_add_steps(64)?;
    let ref_ms = t2.elapsed().as_millis();
    println!("calculator steps: {calc_steps} milliseconds: {calc_ms}");
    let t3 = Instant::now();
    let cli_out = run_calculator_bin()?;
    let cli_ms = t3.elapsed().as_millis();
    let t4 = Instant::now();
    let (body, cells) = load_calculator()?;
    let mut state = match BodyState::new(body, cells, joinn_prim::sealed_natives(), 1) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(r.reason),
    };
    match state.inject("cli_a", 0, text_val("2")?, 0) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Err(r.reason),
    }
    match state.inject("cli_b", 0, text_val("3")?, 1) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Err(r.reason),
    }
    let host_steps = match state.run() {
        Verdict::Ok(_) => state.steps(),
        Verdict::Refused(r) => return Err(r.reason),
    };
    let host_ms = t4.elapsed().as_millis();
    println!("50-cell steps: {fifty_steps} milliseconds: {fifty_ms}");
    println!("reference add(0, 64) steps: {ref_steps} milliseconds: {ref_ms}");
    println!(
        "cli host calculator bytes: {} milliseconds: {cli_ms}",
        cli_out.len()
    );
    println!("test-host calculator steps: {host_steps} milliseconds: {host_ms}");

    let t5 = Instant::now();
    let uni_ports = two_body_universe_ports()?;
    let uni_ms = t5.elapsed().as_millis();
    println!("two-body universe membrane ports: {uni_ports} milliseconds: {uni_ms}");

    let t_load = Instant::now();
    let (parsed, cells) = membrane_load_corpus()?;
    let load_ms = t_load.elapsed().as_millis();
    let t_mem = Instant::now();
    let (bodies, ports, refused) = membrane_compute(&parsed, &cells);
    let mem_ms = t_mem.elapsed().as_millis();
    println!(
        "membrane load+parse milliseconds: {load_ms} compute milliseconds: {mem_ms} bodies: {bodies} ports: {ports} refused: {refused}"
    );
    if refused != 0 {
        return Err(format!(
            "membrane refused {refused} corpus bodies; acceptance is a cell for every genome entry"
        ));
    }
    Ok(())
}

fn two_body_universe_ports() -> Result<usize, String> {
    let root = workspace_root()?;
    let src = fs::read_to_string(root.join("corpus").join("phase5").join("universe.universe"))
        .map_err(|e| e.to_string())?;
    let u = match joinn_link::parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let frames = FrameRegistry::phase1();
    let supplied = load_phase5_bodies()?;
    let cells = supplied
        .values()
        .next()
        .map(|(_, loaded)| loaded.clone())
        .unwrap_or_default();
    let mut by_hash: BTreeMap<Hash, PathBuf> = BTreeMap::new();
    let mut dirs = vec![root.join("corpus")];
    while let Some(dir) = dirs.pop() {
        let rd = fs::read_dir(&dir).map_err(|e| e.to_string())?;
        let mut ents: Vec<_> = rd
            .map(|e| e.map_err(|err| err.to_string()))
            .collect::<Result<Vec<_>, _>>()?;
        ents.sort_by_key(|e| e.file_name());
        for ent in ents {
            let path = ent.path();
            if path.is_dir() {
                dirs.push(path);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("body") {
                continue;
            }
            let body_src = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            if let Verdict::Ok(b) = parse_body(&body_src, &frames) {
                by_hash.insert(hash(&b.coding), path);
            }
        }
    }
    let mut total = 0usize;
    for binding in &u.coding.bodies {
        let Some(path) = by_hash.get(&binding.hash) else {
            return Err(format!("no corpus body for {}", binding.hash.to_hex()));
        };
        let body_src = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let body = match parse_body(&body_src, &frames) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => return Err(r.reason),
        };
        match joinn_link::membrane(&body, &cells) {
            Verdict::Ok(m) => total += m.len(),
            Verdict::Refused(r) => return Err(r.reason),
        }
    }
    match joinn_link::check_law4(&u) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Err(r.reason),
    }
    Ok(total)
}

fn membrane_load_corpus() -> Result<(Vec<joinn_dna::Body>, BTreeMap<Hash, joinn_dna::Cell>), String>
{
    let root = workspace_root()?;
    let frames = FrameRegistry::phase1();
    let mut cells: BTreeMap<Hash, joinn_dna::Cell> = BTreeMap::new();
    let mut parsed = Vec::new();
    for dir_name in ["phase0", "phase2", "phase21", "phase22", "phase3", "phase5"] {
        let dir = root.join("corpus").join(dir_name);
        let rd = match fs::read_dir(&dir) {
            Ok(rd) => rd,
            Err(_) => continue,
        };
        let mut ents: Vec<_> = rd.flatten().collect();
        ents.sort_by_key(|e| e.file_name());
        for ent in ents {
            let path = ent.path();
            if path.is_dir() {
                continue;
            }
            let ext = path.extension().and_then(|e| e.to_str());
            let src = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            if ext == Some("cell") {
                if let Verdict::Ok(cell) = parse_cell(&src, &frames) {
                    let id = hash(&cell.coding);
                    let replace = match cells.get(&id) {
                        Some(prev) => prev.alleles.is_empty() && !cell.alleles.is_empty(),
                        None => true,
                    };
                    if replace {
                        cells.insert(id, cell);
                    }
                }
            } else if ext == Some("body") {
                if let Verdict::Ok(body) = parse_body(&src, &frames) {
                    parsed.push(body);
                }
            }
        }
    }
    Ok((parsed, cells))
}

fn membrane_compute(
    parsed: &[joinn_dna::Body],
    cells: &BTreeMap<Hash, joinn_dna::Cell>,
) -> (usize, usize, usize) {
    let mut bodies = 0usize;
    let mut ports = 0usize;
    let mut refused = 0usize;
    for body in parsed {
        match joinn_link::membrane(body, cells) {
            Verdict::Ok(m) => {
                bodies += 1;
                ports += m.len();
            }
            Verdict::Refused(_) => refused += 1,
        }
    }
    (bodies, ports, refused)
}
