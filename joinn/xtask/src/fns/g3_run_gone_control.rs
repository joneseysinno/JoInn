//! Gate 3 item 7 control: the workspace manifest must not name joinn-run.

use super::{artifact_loads, workspace_root};

pub(crate) fn g3_run_gone_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok(root) = workspace_root() else {
        return true;
    };
    let Ok(toml) = std::fs::read_to_string(root.join("Cargo.toml")) else {
        return true;
    };
    toml.contains("joinn-run")
}
