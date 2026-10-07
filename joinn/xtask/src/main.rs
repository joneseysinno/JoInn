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
        "layers" => fns::layers(),
        "adapters" => fns::adapters(),
        "pick" => fns::pick(),
        "regrow" => fns::regrow(),
        "forces" => fns::forces(),
        "contact" => fns::contact(),
        "grove" => fns::grove(args.collect()),
        "zoom" => fns::zoom(args.collect()),
        "links" => fns::links(args.collect()),
        "grow" => fns::grow(args.collect()),
        "roles" => match args.next() {
            Some(path) => match fns::roles_text(&path) {
                Ok(text) => {
                    let mut out = io::stdout();
                    match out.write_all(text.as_bytes()) {
                        Ok(()) => Ok(()),
                        Err(e) => Err(e.to_string()),
                    }
                }
                Err(e) => Err(e),
            },
            None => Err("usage: cargo xtask roles <path>".into()),
        },
        "layout" => {
            let text = match args.next().as_deref() {
                Some("--all") => fns::layout_all(),
                Some("--universe") => fns::universe_layout_text(args.collect()),
                Some(path) => fns::layout_text(path),
                None => Err(
                    "usage: cargo xtask layout <path> | cargo xtask layout --all | cargo xtask layout --universe <path|grove> [--lens NAME]"
                        .into(),
                ),
            };
            match text {
                Ok(text) => {
                    let mut out = io::stdout();
                    match out.write_all(text.as_bytes()) {
                        Ok(()) => Ok(()),
                        Err(e) => Err(e.to_string()),
                    }
                }
                Err(e) => Err(e),
            }
        }
        "corpus" => match args.next().as_deref() {
            Some("verify") => fns::corpus_verify(),
            Some("rebless") => fns::corpus_rebless(args.collect()),
            _ => Err("usage: cargo xtask corpus verify|rebless".into()),
        },
        "gate" => fns::gate_cmd(args.collect()),
        "check" => fns::check(args.collect()),
        "stop-check" => fns::stop_check(args.collect()),
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
            Some("invariance") => match fns::assay_invariance() {
                Ok(text) => {
                    let mut out = io::stdout();
                    match out.write_all(text.as_bytes()) {
                        Ok(()) => Ok(()),
                        Err(e) => Err(e.to_string()),
                    }
                }
                Err(e) => Err(e),
            },
            Some("--all") => match fns::assay_all() {
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
            None => Err(
                "usage: cargo xtask assay <path> | cargo xtask assay agree | cargo xtask assay invariance | cargo xtask assay --all"
                    .into(),
            ),
        },
        "perf" => fns::perf(),
        "floor" => fns::floor(),
        "decisions" => fns::decisions(),
        "witness" => match args.next() {
            Some(path) => fns::witness(path),
            None => Err("usage: cargo xtask witness <path>".into()),
        },
        "decoration" => match fns::decoration() {
            Ok(text) => {
                let mut out = io::stdout();
                match out.write_all(text.as_bytes()) {
                    Ok(()) => Ok(()),
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(text) => {
                let mut out = io::stdout();
                let _ = out.write_all(text.as_bytes());
                Err("decoration controls failed".into())
            }
        },
        "probe-refusal" => fns::probe_refusal(),
        _ => {
            let _ = writeln!(
                io::stderr(),
                "xtask vocab | modules | layers | adapters | pick | regrow | forces | contact | grove [--seed N] [--out PATH] | zoom | links | roles <path> | layout <path> | layout --all | corpus verify | corpus rebless | gate all | gate 1 | gate 2 | gate 2.1 | gate 2.2 | gate 3 | gate 4 | gate 5 | gate 5.1 | gate 5.2 | gate 6 | gate 7 | gate 7.2 | gate <phase> --item <n> | check [--gates 6,7,7.2] | stop-check [--fresh] | power | agree | assay | assay agree | assay invariance | assay --all | decoration | perf | floor | decisions | witness | probe-refusal"
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

    /// The body of `fns/<name>.rs`'s function `name`, whitespace removed.
    fn gate_fn_body(name: &str) -> Result<String, String> {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("fns")
            .join(format!("{name}.rs"));
        let text = std::fs::read_to_string(&path).map_err(|e| format!("{name}: {e}"))?;
        let text = text.split("#[cfg(test)]").next().unwrap_or_default();
        let sig = format!("fn {name}(");
        let at = text.find(&sig).ok_or(format!("{name}: no `{sig}`"))?;
        let rest = &text[at..];
        let open = rest.find('{').ok_or(format!("{name}: no body"))?;
        Ok(rest[open..]
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect())
    }

    #[test]
    fn no_two_gate_items_share_a_check_or_control() {
        // The four 2.1/2.2 rows that check one fact (`agree_cached().is_ok()`).
        const ONE_FACT: [&str; 4] = ["p21_agree", "p22_see", "p22_true", "p22_roundtrip"];
        let mut checks: Vec<(&str, &str)> = Vec::new();
        let mut controls: Vec<(&str, &str)> = Vec::new();
        for (_, table) in super::fns::legacy_tables() {
            for item in table {
                checks.push((item.name, item.check_name));
                controls.push((item.name, item.control_name));
            }
        }
        for (_, table) in super::fns::opposed_tables() {
            for item in table {
                checks.push((item.name, item.check_name));
                controls.push((item.name, item.control_name));
            }
        }
        assert!(checks.len() > 40, "{} rows", checks.len());
        let mut shared = Vec::new();
        for (kind, fns) in [("check", &checks), ("control", &controls)] {
            let mut bodies = Vec::new();
            for (item, f) in fns.iter() {
                match gate_fn_body(f) {
                    Ok(body) => bodies.push((*item, *f, body)),
                    Err(e) => shared.push(format!("{kind} of {item}: {e}")),
                }
            }
            for (i, (name, f, body)) in bodies.iter().enumerate() {
                for (other, g, other_body) in bodies.iter().skip(i + 1) {
                    if f == g {
                        shared.push(format!("{kind} {name} / {other}: both {f}"));
                    } else if body == other_body && !(ONE_FACT.contains(f) && ONE_FACT.contains(g))
                    {
                        shared.push(format!("{kind} {name} / {other}: {f} and {g} are one body"));
                    }
                }
            }
        }
        assert!(shared.is_empty(), "shared gate functions: {shared:?}");
    }

    #[test]
    fn every_gate_row_names_its_own_functions() {
        let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut bad = Vec::new();
        let walk = super::fns::walk_rs(&manifest.join("src"), &mut |path, text| {
            let lines: Vec<&str> = text.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                for field in ["check", "control"] {
                    let Some(val) = line.trim().strip_prefix(&format!("{field}: ")) else {
                        continue;
                    };
                    let val = val.trim_end_matches(',');
                    if val.starts_with("CheckId::") {
                        continue;
                    }
                    let ident = val.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
                    let want =
                        format!("{field}_name: \"{}\",", if ident { val } else { "closure" });
                    if lines.get(i + 1).map(|l| l.trim()) != Some(want.as_str()) {
                        bad.push(format!("{}:{}: want `{want}`", path.display(), i + 2));
                    }
                }
            }
        });
        assert!(walk.is_ok(), "{}", walk.err().unwrap_or_default());
        assert!(bad.is_empty(), "{bad:?}");
    }
}
