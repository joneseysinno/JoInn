//! Gate 5 item 3 control: calc.sum@9 is no such port, not interior.

use super::load_phase5_bodies;
use super::subject::Subject;
use joinn_frame::Verdict;
use joinn_link::{assemble_universe, bind_bodies};

pub(crate) fn g5_assemble_control(subject: &Subject) -> bool {
    let Subject::Universe(universe) = subject else {
        return true;
    };
    let Ok(supplied) = load_phase5_bodies() else {
        return true;
    };
    let bound = match bind_bodies(universe, &supplied) {
        Verdict::Ok(b) => b,
        Verdict::Refused(_) => return true,
    };
    match assemble_universe(universe, &bound) {
        Verdict::Refused(r) => {
            !(r.reason.contains("calc.sum@9")
                && r.reason.contains("no such port")
                && !r.reason.contains("interior"))
        }
        Verdict::Ok(()) => true,
    }
}
