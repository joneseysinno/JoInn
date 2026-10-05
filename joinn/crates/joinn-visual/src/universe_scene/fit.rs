//! A layout number into a table field.

use joinn_frame::Verdict;

use crate::refuse::refuse;

/// `v` as the field's type, or a refusal naming it.
pub(super) fn fit<T: TryFrom<i64>>(v: i64) -> Verdict<T> {
    match T::try_from(v) {
        Ok(n) => Verdict::Ok(n),
        Err(_) => refuse(format!(
            "scene: {v} does not fit a table field; acceptance is a layout within 32 bits"
        )),
    }
}
