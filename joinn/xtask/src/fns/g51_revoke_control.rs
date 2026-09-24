//! Gate 5.1 item 6 control: the pre-revoke run, read from the subject.

use super::g5_revoke_control;
use super::subject::Subject;

pub(crate) fn g51_revoke_control(subject: &Subject) -> bool {
    g5_revoke_control(subject)
}
