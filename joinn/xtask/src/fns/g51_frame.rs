//! Gate 5.1: every link member's frame is equal.

use super::{load_phase5_bodies, workspace_root};
use joinn_frame::Verdict;
use joinn_link::{bind, check_link_types, parse_universe};
use std::fs;

pub(crate) fn g51_frame() -> bool {
    let Ok(root) = workspace_root() else {
        return false;
    };
    let Ok(store) = load_phase5_bodies() else {
        return false;
    };
    let frames = root
        .join("corpus")
        .join("phase5")
        .join("controls")
        .join("frame_mismatch.universe");
    let Ok(frame_src) = fs::read_to_string(frames) else {
        return false;
    };
    let Verdict::Ok(bad_frame) = parse_universe(&frame_src) else {
        return false;
    };
    let Verdict::Ok(frame_bound) = bind(&bad_frame, &store) else {
        return false;
    };
    let Verdict::Refused(frame) = check_link_types(&bad_frame, &frame_bound) else {
        return false;
    };
    frame.reason.contains("echo.scale@0")
        && frame.reason.contains("Text 1")
        && frame.reason.contains("ℤ 1")
}
