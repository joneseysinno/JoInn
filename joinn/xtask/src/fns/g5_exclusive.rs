//! Gate 5 item 4: exclusivity refuses a body in two systems of one lens.

use super::load_universe_file;
use joinn_frame::Verdict;
use joinn_link::check_lenses;

pub(crate) fn g5_exclusive() -> bool {
    let Ok(u) = load_universe_file("phase5/controls/two_systems.universe") else {
        return false;
    };
    match check_lenses(&u) {
        Verdict::Refused(r) => {
            r.reason.contains("units")
                && r.reason.contains("calculation")
                && r.reason.contains("measurement")
        }
        Verdict::Ok(()) => false,
    }
}
