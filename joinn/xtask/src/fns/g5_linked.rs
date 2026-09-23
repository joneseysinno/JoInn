//! Gate 5 item 1: 2 and 3 cross e0 and units holds 60.

use super::{units_after, workspace_root};
use std::fs;

pub(crate) fn g5_linked() -> bool {
    let Ok(root) = workspace_root() else {
        return false;
    };
    let Ok(src) = fs::read_to_string(root.join("corpus").join("phase5").join("universe.universe"))
    else {
        return false;
    };
    match units_after(&src, true) {
        Ok((1, Some(value))) => value == "60",
        _ => false,
    }
}
