//! Resolve one gate item's control artifact. Does not open it for meaning.

use super::workspace_root;
use std::path::PathBuf;

pub(crate) fn resolve_named(index: usize, name: &str, rel: &str) -> Result<PathBuf, String> {
    let root = workspace_root()?;
    if rel.is_empty() || rel.starts_with('/') || rel.contains('\\') {
        return Err(format!(
            "gate item {index} ({name}) control artifact is not a repo-relative path: {rel}"
        ));
    }
    let mut path = root;
    for part in rel.split('/') {
        if part.is_empty() || part == "." || part == ".." {
            return Err(format!(
                "gate item {index} ({name}) control artifact is not a repo-relative path: {rel}"
            ));
        }
        path.push(part);
    }
    if !path.is_file() {
        return Err(format!(
            "gate item {index} ({name}) control artifact does not exist: {rel}"
        ));
    }
    Ok(path)
}
