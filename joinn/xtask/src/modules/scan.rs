//! Scan crates/ and xtask/src for module-layout violations.

use std::collections::BTreeSet;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use super::count::{allow_modules_reason, production_fns};
use super::fixtures::check_opposed;

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

#[derive(Clone, Copy, PartialEq, Eq)]
enum FileKind {
    FacadeLib,
    FacadeMain,
    CapsuleRoot,
    Leaf,
}

fn classify(path: &Path) -> FileKind {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if name == "lib.rs" {
        return FileKind::FacadeLib;
    }
    if name == "main.rs" {
        return FileKind::FacadeMain;
    }
    // Capsule root: foo.rs beside foo/
    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        if let Some(parent) = path.parent() {
            if parent.join(stem).is_dir() {
                return FileKind::CapsuleRoot;
            }
        }
    }
    FileKind::Leaf
}

/// True when the file's production body is only `impl SomeTrait for T { ... }`.
fn is_trait_impl_only(text: &str) -> bool {
    let stripped = super::count::strip_cfg_test_for_scan(text);
    let mut depth = 0i32;
    let mut saw_trait_impl = false;
    for line in stripped.lines() {
        let t = line.trim_start();
        let at_top = depth == 0;
        if at_top {
            if t.is_empty() || t.starts_with("//") || t.starts_with("use ") || t.starts_with("#!") {
                // fall through to brace count
            } else if t.starts_with("impl ") && t.contains(" for ") {
                saw_trait_impl = true;
            } else if t.starts_with("fn ")
                || t.starts_with("pub fn ")
                || t.starts_with("pub(crate) fn ")
                || t.starts_with("pub(super) fn ")
                || (t.starts_with("impl ") && !t.contains(" for "))
                || t.starts_with("mod ")
            {
                // Free fns, inherent impls, and nested mods break the exception.
                // Accompanying type/const/static defs for the impl'd type are fine.
                return false;
            }
        }
        depth += line.chars().filter(|&c| c == '{').count() as i32;
        depth -= line.chars().filter(|&c| c == '}').count() as i32;
        if depth < 0 {
            depth = 0;
        }
    }
    saw_trait_impl
}

/// True for crate-level integration/trybuild trees (`crates/*/tests/**`).
fn is_under_tests_dir(rel: &str) -> bool {
    rel.contains("/tests/")
}

fn is_enforced(rel: &str, enforced: &BTreeSet<String>) -> bool {
    for e in enforced {
        if e == "xtask" {
            if rel.starts_with("xtask/src/") {
                return true;
            }
        } else {
            let prefix = format!("crates/{e}/");
            if rel.starts_with(&prefix) {
                return true;
            }
        }
    }
    false
}

fn load_lines(path: &Path) -> Result<BTreeSet<String>, String> {
    if !path.exists() {
        return Ok(BTreeSet::new());
    }
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    Ok(text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.replace('\\', "/"))
        .collect())
}

fn workspace_root() -> Result<PathBuf, String> {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    here.parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "xtask has no workspace parent".into())
}

fn walk_rs(dir: &Path, f: &mut impl FnMut(&Path, &str)) -> Result<(), String> {
    if !dir.exists() {
        return Ok(());
    }
    let entries = fs::read_dir(dir).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|n| n == "target") {
                continue;
            }
            walk_rs(&path, f)?;
        } else if path.extension().is_some_and(|e| e == "rs") {
            let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            f(&path, &text);
        }
    }
    Ok(())
}
