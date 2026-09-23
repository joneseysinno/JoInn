//! Opposed fixtures for the modules check.

use std::fs;
use std::path::Path;

use super::count::production_fns;

/// Refuse multi-fn leaf and `mod.rs`; accept one-fn leaf. Fail if either half is blind.
pub fn check_opposed(root: &Path) -> Result<(), String> {
    let fixtures = root.join("xtask").join("module_fixtures");
    let refuse_two = fixtures.join("refuse_two_functions.rs");
    let refuse_mod = fixtures.join("refuse_mod_layout").join("mod.rs");
    let accept_one = fixtures.join("accept_one_function.rs");

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

    Ok(())
}
