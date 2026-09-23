//! Gate 3 item 7: the joinn-run crate is gone.

use super::workspace_root;

pub(crate) fn g3_run_gone() -> bool {
    let Ok(root) = workspace_root() else {
        return false;
    };
    !root.join("crates").join("joinn-run").exists()
}
