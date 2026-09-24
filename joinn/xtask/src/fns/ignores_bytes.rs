//! A control that never looks at the subject. The gate must refuse it.

use super::subject::Subject;

#[allow(dead_code)]
pub(crate) fn ignores_bytes(_: &Subject) -> bool {
    false
}
