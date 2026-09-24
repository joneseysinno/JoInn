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

pub(crate) fn vocab() -> Result<(), String> {
    check_vocab_fixtures()?;
    let root = workspace_root()?;
    let engineering = [
        r"\bHashMap\b",
        r"\bHashSet\b",
        r"\bf32\b",
        r"\bf64\b",
        r"\bthread_rng\b",
    ];
    let concept = [
        r"\bschema\b",
        r"\bmetadata\b",
        r"\bbackend\b",
        r"\bgenotype\b",
    ];
    const BANNED_TURN: &str = "sub subtract minus"; // allow(vocab): turn-ident list
    const FLOOR_WORDS: &[&str] = &[
        "minimal set",           // allow(vocab): banned floor-wording list
        "minimal primitive set", // allow(vocab): banned floor-wording list
        "the eleven",            // allow(vocab): banned floor-wording list
        "the twelve",            // allow(vocab): banned floor-wording list
        "the sixteen",           // allow(vocab): banned floor-wording list
    ];
    let mut hits = Vec::new();

    // Engineering bans: all Rust, unmasked.
    for dir in ["crates", "xtask"] {
        walk_rs(&root.join(dir), &mut |path, text| {
            let p = path.to_string_lossy().replace('\\', "/");
            if p.contains("/vocab_fixtures/") || p.contains("/module_fixtures/") {
                return;
            }
            for (i, line) in text.lines().enumerate() {
                if let Some(reason) = allow_vocab_reason(line) {
                    if reason.is_empty() {
                        hits.push(format!(
                            "{}:{}: bare vocab silencer with no reason",
                            path.display(),
                            i + 1
                        ));
                    }
                    continue;
                }
                for pat in engineering {
                    if line_has(line, pat) {
                        hits.push(format!("{}:{}: {pat}", path.display(), i + 1));
                    }
                }
            }
        })?;
    }

    // Concept-word bans on Rust after masking std calls.
    for dir in ["crates", "xtask"] {
        walk_rs(&root.join(dir), &mut |path, text| {
            let p = path.to_string_lossy().replace('\\', "/");
            if p.contains("/vocab_fixtures/") || p.contains("/module_fixtures/") {
                return;
            }
            for (i, line) in text.lines().enumerate() {
                if allow_vocab_reason(line).is_some() {
                    continue;
                }
                let masked = mask_std_calls(line);
                for pat in concept {
                    if line_has(&masked, pat) {
                        hits.push(format!("{}:{}: {pat}", path.display(), i + 1));
                    }
                }
                for word in BANNED_TURN.split(' ') {
                    if line_has(&masked, word) {
                        hits.push(format!("{}:{}: turn-ident {word}", path.display(), i + 1));
                    }
                }
                let lower = masked.to_ascii_lowercase();
                for w in FLOOR_WORDS {
                    if lower.contains(w) {
                        hits.push(format!("{}:{}: floor-vocab {w}", path.display(), i + 1));
                    }
                }
            }
        })?;
    }

    // Concept-word bans on corpus language files.
    {
        let corpus = root.join("corpus");
        let mut dirs = vec![corpus];
        while let Some(dir) = dirs.pop() {
            let rd = match fs::read_dir(&dir) {
                Ok(rd) => rd,
                Err(_) => continue,
            };
            let mut ents: Vec<_> = rd.flatten().collect();
            ents.sort_by_key(|e| e.file_name());
            for ent in ents {
                let path = ent.path();
                if path.is_dir() {
                    dirs.push(path);
                    continue;
                }
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                if !matches!(ext, "cell" | "body" | "universe" | "desc") {
                    continue;
                }
                let text = match fs::read_to_string(&path) {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                for (i, line) in text.lines().enumerate() {
                    for pat in concept {
                        if line_has(line, pat) {
                            hits.push(format!("{}:{}: {pat}", path.display(), i + 1));
                        }
                    }
                    for word in BANNED_TURN.split(' ') {
                        if line_has(line, word) {
                            hits.push(format!("{}:{}: turn-ident {word}", path.display(), i + 1));
                        }
                    }
                    let lower = line.to_ascii_lowercase();
                    for w in FLOOR_WORDS {
                        if lower.contains(w) {
                            hits.push(format!("{}:{}: floor-vocab {w}", path.display(), i + 1));
                        }
                    }
                }
            }
        }
    }

    walk_rs(&root.join("crates"), &mut |path, text| {
        for (i, line) in text.lines().enumerate() {
            if allow_vocab_reason(line).is_some() {
                continue;
            }
            for word in ["SystemTime", "Instant"] {
                if line_has(line, word) {
                    hits.push(format!("{}:{}: wall-clock {word}", path.display(), i + 1));
                }
            }
        }
    })?;
    walk_rs(
        &root.join("crates").join("joinn-gate"),
        &mut |path, text| {
            for (i, line) in text.lines().enumerate() {
                if allow_vocab_reason(line).is_some() {
                    continue;
                }
                for word in ["BigInt", "BigRational"] {
                    if line_has(line, word) {
                        hits.push(format!(
                            "{}:{}: gate-arithmetic {word}",
                            path.display(),
                            i + 1
                        ));
                    }
                }
            }
        },
    )?;
    for crate_name in ["joinn-live", "joinn-prim"] {
        walk_rs(&root.join("crates").join(crate_name), &mut |path, text| {
            for (i, line) in text.lines().enumerate() {
                if allow_vocab_reason(line).is_some() {
                    continue;
                }
                for word in ["host", "stdin", "stdout", "calculator"] {
                    if line.contains(word) {
                        hits.push(format!("{}:{}: V38 {word}", path.display(), i + 1));
                    }
                }
            }
        })?;
    }
    walk_rs(&root.join("crates"), &mut |path, text| {
        let p = path.to_string_lossy();
        if p.contains("xtask") || p.contains("tests") || p.contains("mutants.rs") {
            return;
        }
        for (i, line) in text.lines().enumerate() {
            if allow_vocab_reason(line).is_some() {
                continue;
            }
            if line.contains("natives_with_mutants") {
                hits.push(format!(
                    "{}:{}: natives_with_mutants outside test/xtask",
                    path.display(),
                    i + 1
                ));
            }
        }
    })?;
    walk_rs(&root.join("crates"), &mut |path, text| {
        let p = path.to_string_lossy().replace('\\', "/");
        if p.contains("/tests/") {
            return;
        }
        let io_ok = p.contains("/joinn-cli/");
        let mut depth = 0i32;
        let mut skip_test = 0i32;
        let mut pending_test = false;
        for (i, line) in text.lines().enumerate() {
            let opens = line.chars().filter(|&c| c == '{').count() as i32;
            let closes = line.chars().filter(|&c| c == '}').count() as i32;
            if line.contains("#[cfg(test)]") {
                pending_test = true;
            }
            if pending_test && opens > 0 {
                skip_test = depth + 1;
                pending_test = false;
            }
            let in_test = skip_test > 0;
            depth += opens - closes;
            if skip_test > 0 && depth < skip_test {
                skip_test = 0;
            }
            if in_test {
                continue;
            }
            if allow_vocab_reason(line).is_some() {
                continue;
            }
            if !io_ok {
                for needle in ["std::io", "std::fs"] {
                    if line.contains(needle) {
                        hits.push(format!("{}:{}: IO {needle}", path.display(), i + 1));
                    }
                }
            }
            if p.contains("/joinn-host/") {
                let render = concat!("fn ", "render");
                if line.contains(render) {
                    hits.push(format!(
                        "{}:{}: String-returning render function",
                        path.display(),
                        i + 1
                    ));
                }
            }
        }
    })?;
    walk_rs(&root.join("crates"), &mut |path, text| {
        let p = path.to_string_lossy().replace('\\', "/");
        if p.contains("/joinn-host/") || p.contains("/tests/") {
            return;
        }
        for (i, line) in text.lines().enumerate() {
            if allow_vocab_reason(line).is_some() {
                continue;
            }
            if line.contains("Description {") {
                hits.push(format!(
                    "{}:{}: Description constructed outside joinn-host",
                    path.display(),
                    i + 1
                ));
            }
        }
    })?;
    if hits.is_empty() {
        println!("vocab: ok");
        Ok(())
    } else {
        Err(format!("vocab: {} hit(s)\n{}", hits.len(), hits.join("\n")))
    }
}
