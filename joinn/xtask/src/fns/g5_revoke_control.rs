//! Gate 5 item 7 control: true when the head's link remains and that body does not fire.

use super::drive_head::drive_head;
use super::head_role::HeadRole;
use super::subject::Subject;

pub(crate) fn g5_revoke_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return false;
    };
    match drive_head(u) {
        Err(_) => false,
        Ok(seen) => matches!(seen.role, HeadRole::One(_)) && !seen.fired,
    }
}
