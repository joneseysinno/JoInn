//! Resolve a finding path from cwd or the repo root above joinn/.

use super::super::workspace_root;
use std::path::{Path, PathBuf};

/// Resolve `arg` to an existing finding file.
pub(crate) fn resolve_path(arg: &str) -> Result<PathBuf, String> {
    let given = PathBuf::from(arg);
    if given.is_file() {
        return Ok(given);
    }
    let ws = workspace_root()?;
    let under_ws = ws.join(arg);
    if under_ws.is_file() {
        return Ok(under_ws);
    }
    if let Some(repo) = ws.parent() {
        let under_repo = repo.join(arg);
        if under_repo.is_file() {
            return Ok(under_repo);
        }
    }
    Err(format!(
        "witness: {} not found; acceptance is a path to a finding file",
        Path::new(arg).display()
    ))
}
