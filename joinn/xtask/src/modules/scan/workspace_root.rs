//! Workspace root from the xtask manifest dir.

use std::path::{Path, PathBuf};

/// Workspace root from the xtask manifest dir.
pub fn workspace_root() -> Result<PathBuf, String> {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    here.parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "xtask has no workspace parent".into())
}
