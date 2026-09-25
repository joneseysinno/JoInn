//! Gate 5 item 3 control: true when assembly refuses naming no such port.

use super::load_phase5_bodies;
use super::subject::Subject;
use joinn_frame::Verdict;
use joinn_link::{assemble_universe, bind};

pub(crate) fn g5_assemble_control(subject: &Subject) -> bool {
    let Subject::Universe(universe) = subject else {
        return false;
    };
    let Ok(store) = load_phase5_bodies() else {
        return false;
    };
    let bound = match bind(universe, &store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(_) => return false,
    };
    match assemble_universe(universe, &bound) {
        Verdict::Refused(r) => r.reason.contains("no such port"),
        Verdict::Ok(()) => false,
    }
}
