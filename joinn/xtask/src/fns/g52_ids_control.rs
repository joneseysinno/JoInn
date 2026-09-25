//! Gate 5.2 item 3 control: true when units does not fire without the path grant.

use super::subject::Subject;
use super::units_after;

pub(crate) fn g52_ids_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return false;
    };
    match units_after(u) {
        Ok((n, _)) => n == 0,
        Err(_) => true,
    }
}
