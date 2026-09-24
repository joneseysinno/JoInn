//! Gate 5 item 5 control: the same pair inside one lens must stay refused.

use super::subject::Subject;
use joinn_frame::Verdict;
use joinn_link::check_lenses;

pub(crate) fn g5_lenses_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return true;
    };
    match check_lenses(u) {
        Verdict::Ok(()) => true,
        Verdict::Refused(_) => false,
    }
}
