//! Gate 5.1 item 4 control: the direction artifact names the in-port tail.

use super::load_phase5_bodies;
use super::subject::Subject;
use joinn_frame::Verdict;
use joinn_link::{bind_bodies, check_link_types};

pub(crate) fn g51_typed_control(subject: &Subject) -> bool {
    let Subject::Universe(universe) = subject else {
        return true;
    };
    let Ok(supplied) = load_phase5_bodies() else {
        return true;
    };
    let Verdict::Ok(bound) = bind_bodies(universe, &supplied) else {
        return true;
    };
    match check_link_types(universe, &bound) {
        Verdict::Refused(r) => {
            !(r.reason.contains("calc.cli_a@0")
                && r.reason.contains("In")
                && r.reason.contains("Out"))
        }
        Verdict::Ok(()) => true,
    }
}
