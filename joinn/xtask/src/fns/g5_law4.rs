//! Gate 5 item 6: Law 4 holds on the well-formed universe.

use super::{load_phase5_bodies, load_universe_file};
use joinn_frame::Verdict;
use joinn_link::{assemble_universe, bind_bodies, check_law4};

pub(crate) fn g5_law4() -> bool {
    let Ok(u) = load_universe_file("phase5/universe.universe") else {
        return false;
    };
    match check_law4(&u) {
        Verdict::Ok(()) => {}
        Verdict::Refused(_) => return false,
    }
    let Ok(supplied) = load_phase5_bodies() else {
        return false;
    };
    let bound = match bind_bodies(&u, &supplied) {
        Verdict::Ok(b) => b,
        Verdict::Refused(_) => return false,
    };
    match assemble_universe(&u, &bound) {
        Verdict::Ok(()) => true,
        Verdict::Refused(_) => false,
    }
}
