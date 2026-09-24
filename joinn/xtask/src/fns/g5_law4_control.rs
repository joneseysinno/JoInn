//! Gate 5 item 6 control: wrong_container fires both Law 4 refusals.

use super::subject::Subject;
use joinn_frame::Verdict;
use joinn_link::{check_law4, law4_refusals};

pub(crate) fn g5_law4_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return true;
    };
    let refusals = law4_refusals(u);
    let both = refusals.len() == 2
        && refusals.iter().any(|r| {
            r.reason.contains("inside") && r.reason.contains("calc") && r.reason.contains("wire")
        })
        && refusals.iter().any(|r| {
            r.reason.contains("wire")
                && r.reason.contains("hyperedge")
                && r.reason.contains("calc")
                && r.reason.contains("units")
        });
    match check_law4(u) {
        Verdict::Refused(_) if both => false,
        _ => true,
    }
}
