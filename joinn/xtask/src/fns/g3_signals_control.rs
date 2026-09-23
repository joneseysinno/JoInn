//! Gate 3 item 6 control: columns_reader.body must still name columns.

use super::{artifact_loads, workspace_root};

pub(crate) fn g3_signals_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok(root) = workspace_root() else {
        return true;
    };
    let Ok(src) = std::fs::read_to_string(
        root.join("corpus")
            .join("phase3")
            .join("columns_reader.body"),
    ) else {
        return true;
    };
    !src.contains("columns")
}
