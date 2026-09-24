//! Fixture control that never looks at its subject.

use crate::fns::subject::Subject;

pub(super) fn ignores_subject(_: &Subject) -> bool {
    false
}
