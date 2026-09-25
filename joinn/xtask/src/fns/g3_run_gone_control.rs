//! Gate 3 item 7 control: the workspace manifest must not name joinn-run.

use super::workspace_root;

pub(crate) fn g3_run_gone_control(_art: &()) -> bool {
    let Ok(root) = workspace_root() else {
        return true;
    };
    let Ok(toml) = std::fs::read_to_string(root.join("Cargo.toml")) else {
        return true;
    };
    toml.contains("joinn-run")
}
