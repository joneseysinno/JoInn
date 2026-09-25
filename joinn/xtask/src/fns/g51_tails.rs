//! Gate 5.1: Tails are out-ports, heads are in-ports.

use super::{load_phase5_bodies, workspace_root};
use joinn_frame::Verdict;
use joinn_link::{bind, check_link_types, parse_universe};
use std::fs;

pub(crate) fn g51_tails() -> bool {
    let Ok(root) = workspace_root() else {
        return false;
    };
    let Ok(store) = load_phase5_bodies() else {
        return false;
    };
    let direction = root
        .join("corpus")
        .join("phase5")
        .join("controls")
        .join("wrong_direction.universe");
    let Ok(direction_src) = fs::read_to_string(direction) else {
        return false;
    };
    let Verdict::Ok(bad_dir) = parse_universe(&direction_src) else {
        return false;
    };
    let Verdict::Ok(dir_bound) = bind(&bad_dir, &store) else {
        return false;
    };
    let Verdict::Refused(dir) = check_link_types(&bad_dir, &dir_bound) else {
        return false;
    };
    dir.reason.contains("calc.cli_a@0")
        && dir.reason.contains("tail")
        && dir.reason.contains("In")
        && dir.reason.contains("Out")
}
