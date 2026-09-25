//! Gate 5.1 tails control: FlipMark makes typing name calc.sum@2.

use super::load_phase5_bodies;
use super::subject::Subject;
use joinn_frame::Verdict;
use joinn_link::{bind, check_link_types};

pub(crate) fn g51_tails_control(subject: &Subject) -> bool {
    let Subject::Universe(universe) = subject else {
        return false;
    };
    let Ok(store) = load_phase5_bodies() else {
        return false;
    };
    let Verdict::Ok(bound) = bind(universe, &store) else {
        return false;
    };
    match check_link_types(universe, &bound) {
        Verdict::Refused(r) => r.reason.contains("calc.sum@2"),
        Verdict::Ok(()) => false,
    }
}
