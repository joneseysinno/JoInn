//! Gate 3 item 3: the host fixture opposes render and IO, then the source is clean.

use super::{walk_rs, workspace_root};

pub(crate) fn g3_value() -> bool {
    let Ok(root) = workspace_root() else {
        return false;
    };
    let fixture = root
        .join("crates")
        .join("joinn-host")
        .join("host_fixtures")
        .join("plant_render.rs");
    let Ok(planted) = std::fs::read_to_string(&fixture) else {
        return false;
    };
    if !planted.contains("fn render") || !planted.contains("std::fs") {
        return false;
    }
    let mut bad = Vec::new();
    let walked = walk_rs(
        &root.join("crates").join("joinn-host").join("src"),
        &mut |path, text| {
            if text.contains("fn render") {
                bad.push(format!("{}: fn render", path.display()));
            }
            if text.contains("std::fs") || text.contains("std::io") {
                bad.push(format!("{}: std::fs", path.display()));
            }
        },
    );
    if !bad.is_empty() {
        println!("{}", bad.join("\n"));
    }
    walked.is_ok() && bad.is_empty()
}
