//! Gate 5.2 control: true when binding refuses naming calc and the declared hash.

use super::load_phase5_bodies;
use super::subject::Subject;
use joinn_frame::Verdict;
use joinn_link::bind;

pub(crate) fn g52_bind_control(subject: &Subject) -> bool {
    let Subject::Universe(universe) = subject else {
        return false;
    };
    let Ok(store) = load_phase5_bodies() else {
        return false;
    };
    let Some(calc) = universe.coding.bodies.iter().find(|b| b.alias == "calc") else {
        return false;
    };
    let declared = calc.hash.to_hex();
    match bind(universe, &store) {
        Verdict::Refused(r) => r.reason.contains("calc") && r.reason.contains(&declared),
        Verdict::Ok(_) => false,
    }
}
