//! Gate 5.1 item 2 control: the transcript's prefix is the calculator golden.

use super::{artifact_loads, workspace_root};
use std::fs;

pub(crate) fn g51_hosts_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok(root) = workspace_root() else {
        return true;
    };
    let Ok(calc) = fs::read(
        root.join("corpus")
            .join("transcripts")
            .join("calculator.txt"),
    ) else {
        return true;
    };
    if art.bytes.len() < calc.len() {
        return true;
    }
    art.bytes[..calc.len()] != calc
}
