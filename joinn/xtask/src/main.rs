//! Gate runner, corpus tools, vocab lint, power, agree, perf, modules.

#![forbid(unsafe_code)]

mod fns;
mod modules;

use std::env;
use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let cmd = args.next().unwrap_or_else(|| "help".into());
    let result = match cmd.as_str() {
        "vocab" => fns::vocab(),
        "modules" => modules::run(),
        "corpus" => match args.next().as_deref() {
            Some("verify") => fns::corpus_verify(),
            Some("rebless") => fns::corpus_rebless(args.collect()),
            _ => Err("usage: cargo xtask corpus verify|rebless".into()),
        },
        "gate" => match args.next().as_deref() {
            Some("all") => fns::gate_all(),
            Some("1") => fns::gate_one(fns::PHASE_LABELS[1]).and_then(fns::require_full),
            Some("2") => fns::gate_two(fns::PHASE_LABELS[2]).and_then(fns::require_full),
            Some("2.1") => fns::gate_two_one(fns::PHASE_LABELS[3]).and_then(fns::require_full),
            Some("2.2") => fns::gate_two_two(fns::PHASE_LABELS[4]).and_then(fns::require_full),
            Some("3") => fns::gate_three(fns::PHASE_LABELS[5]).and_then(fns::require_full),
            Some("5") => fns::gate_five(fns::PHASE_LABELS[6]).and_then(fns::require_full),
            Some("5.1") => fns::gate_five_one(fns::PHASE_LABELS[7]).and_then(fns::require_full),
            Some("5.2") => fns::gate_five_two(fns::PHASE_LABELS[8]).and_then(fns::require_full),
            _ => Err("usage: cargo xtask gate all|1|2|2.1|2.2|3|5|5.1|5.2".into()),
        },
        "power" => fns::power(),
        "agree" => fns::agree(),
        "assay" => match args.next().as_deref() {
            Some("agree") => match fns::assay_agree() {
                Ok(text) => {
                    let mut out = io::stdout();
                    match out.write_all(text.as_bytes()) {
                        Ok(()) => Ok(()),
                        Err(e) => Err(e.to_string()),
                    }
                }
                Err(e) => Err(e),
            },
            Some(path) => match fns::assay(path) {
                Ok(text) => {
                    let mut out = io::stdout();
                    match out.write_all(text.as_bytes()) {
                        Ok(()) => Ok(()),
                        Err(e) => Err(e.to_string()),
                    }
                }
                Err(e) => Err(e),
            },
            None => Err("usage: cargo xtask assay <path> | cargo xtask assay agree".into()),
        },
        "perf" => fns::perf(),
        "floor" => fns::floor(),
        "decisions" => fns::decisions(),
        "witness" => match args.next() {
            Some(path) => fns::witness(path),
            None => Err("usage: cargo xtask witness <path>".into()),
        },
        "probe-refusal" => fns::probe_refusal(),
        _ => {
            let _ = writeln!(
                io::stderr(),
                "xtask vocab | modules | corpus verify | corpus rebless | gate all | gate 1 | gate 2 | gate 2.1 | gate 2.2 | gate 3 | gate 5 | gate 5.1 | gate 5.2 | power | agree | assay | assay agree | perf | floor | decisions | witness | probe-refusal"
            );
            Ok(())
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            let _ = writeln!(io::stderr(), "{e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn write_lock_has_no_literal_gate_score() {
        let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let src = manifest.join("src");
        let lits = [
            format!("{}/{}", 8, 8),
            format!("{}/{}", 9, 9),
            format!("{}/{}", 4, 4),
        ];
        let mut hits = Vec::new();
        let walk = super::fns::walk_rs(&src, &mut |path, text| {
            for (i, line) in text.lines().enumerate() {
                for lit in &lits {
                    if line.contains(lit) {
                        hits.push(format!("{}:{}: {lit}", path.display(), i + 1));
                    }
                }
            }
        });
        assert!(walk.is_ok(), "{}", walk.err().unwrap_or_default());
        assert!(hits.is_empty(), "literal gate score: {hits:?}");
    }

    #[test]
    fn xtask_does_not_write_findings() {
        assert!(
            !super::fns::xtask_writes_findings(),
            "xtask must not write under docs/Findings"
        );
    }

    #[test]
    fn source_checks_scan_a_tree() {
        let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let src = manifest.join("src");
        let mut include_hits = Vec::new();
        let walk = super::fns::walk_rs(&src, &mut |path, text| {
            for (i, line) in text.lines().enumerate() {
                let needle = "include_str!(\"";
                if let Some(idx) = line.find(needle) {
                    let rest = &line[idx + needle.len()..];
                    if let Some(end) = rest.find("\")") {
                        let included = &rest[..end];
                        if included.ends_with(".rs") {
                            include_hits.push(format!("{}:{}: {included}", path.display(), i + 1));
                        }
                    }
                }
            }
        });
        assert!(walk.is_ok(), "{}", walk.err().unwrap_or_default());
        assert!(
            include_hits.is_empty(),
            "include_str of a source file is only allowed when that file is the check's subject; got {include_hits:?}"
        );
    }

    #[test]
    fn bound_zero_artifact_is_refused_by_name() {
        assert!(
            super::fns::p22_bound(),
            "zero-bound SealSpec must be refused naming the seal"
        );
        assert!(
            !super::fns::p22_bound_control(&()),
            "control must stay refused while the loading path refuses bound 0"
        );
    }

    #[test]
    fn no_file_but_gate_all_says_phase_5() {
        let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let src = manifest.join("src");
        let mut hits = Vec::new();
        let walk = super::fns::walk_rs(&src, &mut |path, text| {
            if path.file_name().is_some_and(|n| n == "gate_all.rs") {
                return;
            }
            for (i, line) in text.lines().enumerate() {
                let needle = format!("phase {}", 5);
                if line.contains(&needle) {
                    hits.push(format!("{}:{}", path.display(), i + 1));
                }
            }
        });
        assert!(walk.is_ok(), "{}", walk.err().unwrap_or_default());
        assert!(hits.is_empty(), "phase label literal: {hits:?}");
    }

    #[test]
    fn bodies_are_not_fetched_by_alias_before_binding() {
        let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut hits = Vec::new();
        for dir in [
            manifest.join("src"),
            manifest
                .join("..")
                .join("crates")
                .join("joinn-link")
                .join("src"),
        ] {
            let walk = super::fns::walk_rs(&dir, &mut |path, text| {
                let needle = format!("bodies.get(&binding.{})", "alias");
                if text.contains(&needle) {
                    hits.push(path.display().to_string());
                }
            });
            assert!(walk.is_ok(), "{}", walk.err().unwrap_or_default());
        }
        assert!(hits.is_empty(), "alias lookup before binding: {hits:?}");
    }

    #[test]
    fn no_two_gate_items_share_a_check_or_control() {
        let legacy = [
            super::fns::gate_one_items(),
            super::fns::gate_two_items(),
            super::fns::gate_two_one_items(),
            super::fns::gate_two_two_items(),
            super::fns::gate_three_items(),
        ];
        let non_legacy = [
            super::fns::gate_five_items(),
            super::fns::gate_five_one_items(),
            super::fns::gate_five_two_items(),
        ];
        let mut checks = Vec::new();
        let mut controls = Vec::new();
        for table in legacy {
            for item in table {
                checks.push((item.name, item.check as usize));
                controls.push((item.name, item.control as usize));
            }
        }
        for table in non_legacy {
            for item in table {
                checks.push((item.name, item.check as usize));
                controls.push((item.name, item.control as usize));
            }
        }
        let mut shared = Vec::new();
        for (i, (name, ptr)) in checks.iter().enumerate() {
            for (other, other_ptr) in checks.iter().skip(i + 1) {
                if ptr == other_ptr {
                    shared.push(format!("check {name} / {other}"));
                }
            }
        }
        for (i, (name, ptr)) in controls.iter().enumerate() {
            for (other, other_ptr) in controls.iter().skip(i + 1) {
                if ptr == other_ptr {
                    shared.push(format!("control {name} / {other}"));
                }
            }
        }
        assert!(shared.is_empty(), "shared gate functions: {shared:?}");
    }
}
