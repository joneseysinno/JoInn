//! Gate 5 item 1 control: true when units does not fire.

use super::subject::Subject;
use super::units_after;

pub(crate) fn g5_linked_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return true;
    };
    match units_after(u, Some("e0")) {
        Ok((n, _)) => n == 0,
        Err(_) => true,
    }
}
