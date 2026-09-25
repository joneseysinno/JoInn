//! Gate 5 item 1: 2 and 3 cross e0 and units holds 60.

use super::{load_universe_file, units_after};

pub(crate) fn g5_linked() -> bool {
    let Ok(u) = load_universe_file("phase5/universe.universe") else {
        return false;
    };
    match units_after(&u) {
        Ok((1, Some(value))) => value == "60",
        _ => false,
    }
}
