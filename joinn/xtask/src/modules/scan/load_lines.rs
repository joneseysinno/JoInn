//! Load a comment-stripping line set from a fixture file.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

/// Load non-empty, non-comment lines from `path` into a set.
pub fn load_lines(path: &Path) -> Result<BTreeSet<String>, String> {
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
