//! Gate 5 item 8: the far side does not carry the refusing body's words.

use super::double_delivery::double_delivery;
use super::workspace_root;
use joinn_link::format_link_refusal;
use std::fs;

pub(crate) fn g5_locality() -> bool {
    let Ok(root) = workspace_root() else {
        return false;
    };
    let Ok(secret) = fs::read_to_string(
        root.join("corpus")
            .join("phase5")
            .join("controls")
            .join("inner_reason.txt"),
    ) else {
        return false;
    };
    let secret = secret.trim();
    if secret.is_empty() {
        return false;
    }
    let Ok(delivery) = double_delivery() else {
        return false;
    };
    let far_lines: Vec<String> = delivery.far_side.iter().map(format_link_refusal).collect();
    if far_lines.is_empty() {
        return false;
    }
    if far_lines.iter().any(|line| line.contains(secret)) {
        return false;
    }
    delivery.label.contains(secret)
}
