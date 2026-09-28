//! The workspace members listed in the root `Cargo.toml`, in file order.

use std::fs;
use std::path::{Path, PathBuf};

/// Every quoted path between `members = [` and `]`, joined to `root`.
pub(crate) fn members(root: &Path) -> Result<Vec<PathBuf>, String> {
    let path = root.join("Cargo.toml");
    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let start = text
        .find("members = [")
        .ok_or_else(|| format!("{}: no workspace members", path.display()))?;
    let rest = &text[start + "members = [".len()..];
    let end = rest
        .find(']')
        .ok_or_else(|| format!("{}: members list is not closed", path.display()))?;
    let mut out = Vec::new();
    for item in rest[..end].split(',') {
        let item = item.trim().trim_matches('"');
        if !item.is_empty() {
            out.push(root.join(item));
        }
    }
    Ok(out)
}
