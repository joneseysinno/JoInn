//! Run the opposed fixtures and the tree scan.

use std::collections::BTreeSet;
use std::io::{self, Write};

use super::super::count::{allow_modules_reason, production_fns};
use super::super::fixtures::check_opposed;
use super::classify::{FileKind, classify};
use super::is_enforced::is_enforced;
use super::is_trait_impl_only::is_trait_impl_only;
use super::is_under_tests_dir::is_under_tests_dir;
use super::load_lines::load_lines;
use super::walk_rs::walk_rs;
use super::workspace_root::workspace_root;

/// Run the opposed fixtures and the tree scan.
pub fn run() -> Result<(), String> {
    let root = workspace_root()?;
    check_opposed(&root)?;

    let enforced = load_lines(
        &root
            .join("xtask")
            .join("module_fixtures")
            .join("enforced.txt"),
    )?;
    let grandfather = load_lines(
        &root
            .join("xtask")
            .join("module_fixtures")
            .join("grandfather.txt"),
    )?;

    let mut hits = Vec::new();
    let mut seen_grandfather: BTreeSet<String> = BTreeSet::new();

    for dir in [root.join("crates"), root.join("xtask").join("src")] {
        walk_rs(&dir, &mut |path, text| {
            let rel = match path.strip_prefix(&root) {
                Ok(r) => r.to_string_lossy().replace('\\', "/"),
                Err(_) => path.to_string_lossy().replace('\\', "/"),
            };

            if path.file_name().is_some_and(|n| n == "mod.rs") {
                // Planted fixture under module_fixtures is outside xtask/src.
                hits.push(format!("{rel}: mod.rs is refused — use foo.rs + foo/"));
                return;
            }

            if !is_enforced(&rel, &enforced) {
                return;
            }

            let silenced = text
                .lines()
                .any(|l| allow_modules_reason(l).is_some_and(|r| !r.is_empty()));
            if text.lines().any(|l| allow_modules_reason(l) == Some("")) {
                hits.push(format!("{rel}: bare modules silencer with no reason"));
                return;
            }
            if silenced {
                return;
            }

            let on_gf = grandfather.contains(&rel);
            if on_gf {
                seen_grandfather.insert(rel.clone());
            }

            // Crate-level `tests/` is for trybuild/integration, not capsule leaves.
            // Still refuse mod.rs (checked above). Exempt from the one-fn rule.
            if is_under_tests_dir(&rel) {
                if on_gf {
                    hits.push(format!(
                        "{rel}: stale grandfather — tests/ is exempt from the leaf rule"
                    ));
                }
                return;
            }

            let kind = classify(path);
            let n = production_fns(text);
            let trait_impl_leaf = kind == FileKind::Leaf && is_trait_impl_only(text);
            let ok = match kind {
                FileKind::FacadeLib => n == 0,
                FileKind::FacadeMain => n <= 1, // main only
                // Capsule roots hold types + getters; leaf files are the one-fn rule.
                FileKind::CapsuleRoot => true,
                // Trait-impl exception: required methods of one `impl Trait for T` may share a file.
                FileKind::Leaf => n <= 1 || trait_impl_leaf,
            };

            if on_gf {
                if ok && kind == FileKind::Leaf && n <= 1 {
                    hits.push(format!(
                        "{rel}: stale grandfather — file already obeys the leaf rule"
                    ));
                }
                return;
            }

            if !ok {
                let msg = match kind {
                    FileKind::FacadeLib => {
                        format!("{rel}: lib.rs facade has {n} production fn(s); want 0")
                    }
                    FileKind::FacadeMain => {
                        format!("{rel}: main.rs has {n} production fn(s); want main only")
                    }
                    FileKind::CapsuleRoot => {
                        format!(
                            "{rel}: capsule root has {n} production fn(s); want types/mod/getters only"
                        )
                    }
                    FileKind::Leaf => {
                        format!("{rel}: leaf has {n} production fn(s); want at most 1")
                    }
                };
                hits.push(msg);
            }
        })?;
    }

    for g in &grandfather {
        // tests/ paths are exempt — they must not linger on the grandfather list.
        if is_under_tests_dir(g) {
            if !seen_grandfather.contains(g) {
                hits.push(format!(
                    "{g}: stale grandfather — tests/ is exempt from the leaf rule"
                ));
            }
            continue;
        }
        if is_enforced(g, &enforced) && !seen_grandfather.contains(g) {
            hits.push(format!(
                "{g}: grandfather entry not found under enforced crates (stale or mistyped)"
            ));
        }
    }

    if hits.is_empty() {
        let _ = writeln!(
            io::stdout(),
            "modules: ok (enforced {} crate(s))",
            enforced.len()
        );
        Ok(())
    } else {
        for h in &hits {
            let _ = writeln!(io::stderr(), "{h}");
        }
        Err(format!("modules: {} hit(s)", hits.len()))
    }
}
