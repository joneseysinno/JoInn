//! Gate 5.1 frame control: SwapBinding makes typing name Text 1 and ℤ 1.

use super::load_phase5_bodies;
use super::subject::Subject;
use joinn_frame::Verdict;
use joinn_link::{bind_bodies, check_link_types};

pub(crate) fn g51_frame_control(subject: &Subject) -> bool {
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
        Verdict::Refused(r) => r.reason.contains("Text 1") && r.reason.contains("ℤ 1"),
        Verdict::Ok(()) => false,
    }
}
