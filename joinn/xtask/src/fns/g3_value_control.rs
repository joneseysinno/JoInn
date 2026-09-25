//! Gate 3 item 3 control: the fixture must still contain a render and an IO plant.

use super::workspace_root;

pub(crate) fn g3_value_control(_art: &()) -> bool {
    let Ok(root) = workspace_root() else {
        return true;
    };
    let Ok(src) = std::fs::read_to_string(
        root.join("crates")
            .join("joinn-host")
            .join("host_fixtures")
            .join("plant_render.rs"),
    ) else {
        return true;
    };
    !src.contains("fn render") || !src.contains("std::fs")
}
