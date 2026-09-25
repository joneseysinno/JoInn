//! Gate 5.2 item 1: binding is by store.

use super::{load_phase5_bodies, workspace_root};
use joinn_frame::Verdict;
use joinn_link::{bind, parse_universe};
use std::fs;

pub(crate) fn g52_bind() -> bool {
    let Ok(root) = workspace_root() else {
        return false;
    };
    let Ok(store) = load_phase5_bodies() else {
        return false;
    };
    for rel in [
        "phase5/universe.universe",
        "phase5/controls/alias_is_local.universe",
    ] {
        let Ok(src) = fs::read_to_string(root.join("corpus").join(rel)) else {
            return false;
        };
        let Verdict::Ok(universe) = parse_universe(&src) else {
            return false;
        };
        if !matches!(bind(&universe, &store), Verdict::Ok(_)) {
            return false;
        }
    }
    true
}
