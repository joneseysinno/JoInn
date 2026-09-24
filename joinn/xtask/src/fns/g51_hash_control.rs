//! Gate 5.1 control: true when binding refuses naming calc.

use super::load_phase5_bodies;
use super::subject::Subject;
use joinn_frame::Verdict;
use joinn_link::bind_bodies;

pub(crate) fn g51_hash_control(subject: &Subject) -> bool {
    let Subject::Universe(universe) = subject else {
        return true;
    };
    let Ok(supplied) = load_phase5_bodies() else {
        return true;
    };
    match bind_bodies(universe, &supplied) {
        Verdict::Refused(r) => r.reason.contains("calc"),
        Verdict::Ok(_) => false,
    }
}
