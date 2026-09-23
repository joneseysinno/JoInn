//! Gate 3 item 5 control: calculator.body must still name both inputs.

use super::{artifact_loads, workspace_root};

pub(crate) fn g3_intent_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok(root) = workspace_root() else {
        return true;
    };
    let Ok(src) = std::fs::read_to_string(root.join("corpus").join("phase2").join("calculator.body"))
    else {
        return true;
    };
    !src.contains("cli_a") || !src.contains("cli_b")
}
