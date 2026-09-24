//! Gate 5 item 4 control: exclusivity refuses units in two systems of one lens.

use super::subject::Subject;
use joinn_frame::Verdict;
use joinn_link::check_lenses;

pub(crate) fn g5_exclusive_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return true;
    };
    match check_lenses(u) {
        Verdict::Refused(r) => {
            !(r.reason.contains("units")
                && r.reason.contains("calculation")
                && r.reason.contains("measurement"))
        }
        Verdict::Ok(()) => true,
    }
}
