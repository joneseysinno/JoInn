//! Gate 3 item 2 control: planted_desc differs from the golden in the value field.

use super::{artifact_loads, first_desc_diff, workspace_root};

pub(crate) fn g3_descriptions_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok(root) = workspace_root() else {
        return true;
    };
    let Ok(golden) = std::fs::read_to_string(
        root.join("corpus")
            .join("descriptions")
            .join("calculator.desc"),
    ) else {
        return true;
    };
    let Ok(planted) = std::fs::read_to_string(
        root.join("corpus")
            .join("phase3")
            .join("controls")
            .join("planted_desc.desc"),
    ) else {
        return true;
    };
    let golden = golden.replace("\r\n", "\n");
    let planted = planted.replace("\r\n", "\n");
    if planted == golden {
        return true;
    }
    !matches!(first_desc_diff(&golden, &planted), Some(field) if field == "value")
}
