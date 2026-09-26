//! Gate 5 item 5 control: true when fewer than two lenses place the head body.

use super::drive_head::drive_head;
use super::head_role::HeadRole;
use super::subject::Subject;

pub(crate) fn g5_lenses_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return false;
    };
    let Ok(seen) = drive_head(u) else {
        return false;
    };
    let HeadRole::One(head) = seen.role else {
        return false;
    };
    let placing = u
        .coding
        .lenses
        .iter()
        .filter(|lens| {
            lens.galaxies.iter().any(|galaxy| {
                galaxy
                    .systems
                    .iter()
                    .any(|system| system.bodies.iter().any(|alias| alias == &head))
            })
        })
        .count();
    placing < 2
}
