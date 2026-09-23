//! Whether any file under `xtask/src` writes `docs/Findings`.

use super::{walk_rs, workspace_root};

/// True if any `.rs` file under `xtask/src` names a `Findings` path join.
pub(crate) fn xtask_writes_findings() -> bool {
    let Ok(root) = workspace_root() else {
        return true;
    };
    let mut found = false;
    let src = root.join("xtask").join("src");
    let _ = walk_rs(&src, &mut |_, text| {
        if text.contains("join(\"Findings\")") {
            found = true;
        }
    });
    found
}
