//! Gate 5 item 6 control: true when Law 4 names both bodies and hyperedge.

use super::subject::Subject;
use joinn_frame::Verdict;
use joinn_link::check_law4;

pub(crate) fn g5_law4_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return false;
    };
    match check_law4(u) {
        Verdict::Refused(r) => {
            r.reason.contains("calc")
                && r.reason.contains("units")
                && r.reason.contains("hyperedge")
        }
        Verdict::Ok(()) => false,
    }
}
