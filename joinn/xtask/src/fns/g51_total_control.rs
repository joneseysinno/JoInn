//! Gate 5.1 item 5: a missing cell is refused by name.

use super::subject::Subject;
use joinn_frame::Verdict;
use joinn_link::membrane;
use std::collections::BTreeMap;

pub(crate) fn g51_total_control(subject: &Subject) -> bool {
    let Subject::Body(body) = subject else {
        return true;
    };
    match membrane(body, &BTreeMap::new()) {
        Verdict::Refused(r) => !(r.reason.contains("scale") && r.reason.contains("12b6")),
        Verdict::Ok(_) => true,
    }
}
