//! Gate 5 item 7 control: true when units does not fire before any revoke.

use super::subject::Subject;
use super::units_after;

pub(crate) fn g5_revoke_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return true;
    };
    match units_after(u, Some("path")) {
        Ok((n, _)) => n == 0,
        Err(_) => true,
    }
}
