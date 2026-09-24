//! Gate 5 item 6 control: true when Law 4 refuses naming the container.

use super::subject::Subject;
use joinn_frame::Verdict;
use joinn_link::check_law4;

pub(crate) fn g5_law4_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return true;
    };
    match check_law4(u) {
        Verdict::Refused(r) => {
            r.reason.contains("wire")
                || r.reason.contains("hyperedge")
                || r.reason.contains("acceptance is a wire")
        }
        Verdict::Ok(()) => false,
    }
}
