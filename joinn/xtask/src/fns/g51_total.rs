//! Gate 5.1 item: calculator's surface is measured, and a missing cell is named.

use super::workspace_root;
use joinn_dna::parse_body;
use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::surface;
use std::fs;

pub(crate) fn g51_total() -> bool {
    let Ok(root) = workspace_root() else {
        return false;
    };
    let frames = FrameRegistry::phase1();
    let Ok(calc_src) =
        fs::read_to_string(root.join("corpus").join("phase2").join("calculator.body"))
    else {
        return false;
    };
    let Ok(missing_src) = fs::read_to_string(
        root.join("corpus")
            .join("phase52")
            .join("controls")
            .join("missing_cell.body"),
    ) else {
        return false;
    };
    let Verdict::Ok(calc) = parse_body(&calc_src, &frames) else {
        return false;
    };
    let Verdict::Ok(missing) = parse_body(&missing_src, &frames) else {
        return false;
    };
    let Ok(supplied) = super::load_phase5_bodies() else {
        return false;
    };
    let Some(cells) = supplied.any_cells() else {
        return false;
    };
    let Verdict::Ok(mem) = surface(&calc, cells) else {
        return false;
    };
    if mem.is_empty() {
        return false;
    }
    match surface(&missing, cells) {
        Verdict::Refused(r) => r.reason.contains("orphan"),
        Verdict::Ok(_) => false,
    }
}
