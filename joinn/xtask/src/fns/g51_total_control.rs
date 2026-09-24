//! Gate 5.1 control: true when ∂ is computed with no refusal.

use super::load_phase5_bodies;
use super::subject::Subject;
use joinn_frame::Verdict;
use joinn_link::membrane;

pub(crate) fn g51_total_control(subject: &Subject) -> bool {
    let Subject::Body(body) = subject else {
        return true;
    };
    let Ok(supplied) = load_phase5_bodies() else {
        return true;
    };
    let Some((_, cells)) = supplied.values().next() else {
        return true;
    };
    matches!(membrane(body, cells), Verdict::Ok(_))
}
