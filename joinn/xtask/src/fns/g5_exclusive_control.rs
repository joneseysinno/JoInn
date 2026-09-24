//! Gate 5 item 4 control: true when exclusivity refuses naming units.

use super::subject::Subject;
use joinn_frame::Verdict;
use joinn_link::check_lenses;

pub(crate) fn g5_exclusive_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return true;
    };
    match check_lenses(u) {
        Verdict::Refused(r) => r.reason.contains("units"),
        Verdict::Ok(()) => false,
    }
}
