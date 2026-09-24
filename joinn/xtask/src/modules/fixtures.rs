//! Opposed fixtures for the modules check.

use std::fs;
use std::path::Path;

use super::count::production_fns;
use super::scan::{FileKind, classify};

/// Refuse multi-fn leaf, empty-capsule loophole, and `mod.rs`; accept one-fn leaf.
pub fn check_opposed(root: &Path) -> Result<(), String> {
    let fixtures = root.join("xtask").join("module_fixtures");
    let refuse_two = fixtures.join("refuse_two_functions.rs");
    let refuse_mod = fixtures.join("refuse_mod_layout").join("mod.rs");
    let accept_one = fixtures.join("accept_one_function.rs");
    let refuse_empty = fixtures.join("refuse_empty_capsule.rs");

    let two = fs::read_to_string(&refuse_two).map_err(|e| e.to_string())?;
    if production_fns(&two) < 2 {
        return Err(
            "modules: blind refuse — refuse_two_functions.rs does not have two production fns"
                .into(),
        );
    }

    if !refuse_mod.exists() {
        return Err("modules: blind refuse — refuse_mod_layout/mod.rs is missing".into());
    }
    if refuse_mod.file_name().is_none_or(|n| n != "mod.rs") {
        return Err("modules: blind refuse — planted mod.rs is not named mod.rs".into());
    }

    let one = fs::read_to_string(&accept_one).map_err(|e| e.to_string())?;
    if production_fns(&one) != 1 {
        return Err(
            "modules: blind accept — accept_one_function.rs must have exactly one production fn"
                .into(),
        );
    }

    let empty = fs::read_to_string(&refuse_empty).map_err(|e| e.to_string())?;
    if production_fns(&empty) < 2 {
        return Err(
            "modules: blind refuse — refuse_empty_capsule.rs does not have two production fns"
                .into(),
        );
    }
    let kind = classify(&refuse_empty);
    if kind != FileKind::Leaf {
        return Err(format!(
            "modules: blind refuse — refuse_empty_capsule.rs beside an empty-of-rs folder must classify as Leaf, got {kind:?}"
        ));
    }
    println!(
        "modules fixture: refuse_empty_capsule.rs is Leaf with {} production fn(s)",
        production_fns(&empty)
    );

    Ok(())
}
