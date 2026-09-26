//! Gate 5 item 1 control: true when no value reaches the head body.

use super::drive_head::drive_head;
use super::head_role::HeadRole;
use super::subject::Subject;

pub(crate) fn g5_linked_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return false;
    };
    match drive_head(u) {
        Err(_) => false,
        Ok(seen) => match seen.role {
            HeadRole::Absent => true,
            HeadRole::Split => false,
            HeadRole::One(_) => !seen.value_reached,
        },
    }
}
