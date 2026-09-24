//! Gate 5 item 7 control: before revoke, units fires.

use super::subject::Subject;
use super::units_after;

pub(crate) fn g5_revoke_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return true;
    };
    match units_after(u, true) {
        Ok((1, Some(value))) if value == "60" => false,
        _ => true,
    }
}
