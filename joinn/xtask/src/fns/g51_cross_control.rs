//! Gate 5.1 item 1 control: the unlinked universe, read from the subject.

use super::g5_linked_control;
use super::subject::Subject;

pub(crate) fn g51_cross_control(subject: &Subject) -> bool {
    g5_linked_control(subject)
}
