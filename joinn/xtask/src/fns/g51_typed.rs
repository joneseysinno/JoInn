//! Gate 5.1 item 4: direction and frame each name the member.

use super::{load_phase5_bodies, workspace_root};
use joinn_frame::Verdict;
use joinn_link::{bind_bodies, check_link_types, parse_universe};
use std::fs;

pub(crate) fn g51_typed() -> bool {
    let Ok(root) = workspace_root() else {
        return false;
    };
    let Ok(supplied) = load_phase5_bodies() else {
        return false;
    };
    let direction = root
        .join("corpus")
        .join("phase5")
        .join("controls")
        .join("wrong_direction.universe");
    let frames = root
        .join("corpus")
        .join("phase5")
        .join("controls")
        .join("frame_mismatch.universe");
    let Ok(direction_src) = fs::read_to_string(direction) else {
        return false;
    };
    let Ok(frame_src) = fs::read_to_string(frames) else {
        return false;
    };
    let Verdict::Ok(bad_dir) = parse_universe(&direction_src) else {
        return false;
    };
    let Verdict::Ok(bad_frame) = parse_universe(&frame_src) else {
        return false;
    };
    let Verdict::Ok(dir_bound) = bind_bodies(&bad_dir, &supplied) else {
        return false;
    };
    let Verdict::Ok(frame_bound) = bind_bodies(&bad_frame, &supplied) else {
        return false;
    };
    let Verdict::Refused(dir) = check_link_types(&bad_dir, &dir_bound) else {
        return false;
    };
    let Verdict::Refused(frame) = check_link_types(&bad_frame, &frame_bound) else {
        return false;
    };
    dir.reason.contains("calc.cli_a@0")
        && dir.reason.contains("tail")
        && dir.reason.contains("In")
        && dir.reason.contains("Out")
        && frame.reason.contains("echo.scale@0")
        && frame.reason.contains("Text 1")
        && frame.reason.contains("ℤ 1")
}
