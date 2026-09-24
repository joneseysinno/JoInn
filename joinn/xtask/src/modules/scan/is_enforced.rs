//! Whether a relative path falls under an enforced crate.

use std::collections::BTreeSet;

/// Whether a relative path falls under an enforced crate.
pub fn is_enforced(rel: &str, enforced: &BTreeSet<String>) -> bool {
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
