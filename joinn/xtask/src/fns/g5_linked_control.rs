//! Gate 5 item 1 control: without e0, units does not fire.

use super::subject::Subject;
use super::units_after;

pub(crate) fn g5_linked_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return true;
    };
    match units_after(u, false) {
        Ok((0, _)) => false,
        _ => true,
    }
}
