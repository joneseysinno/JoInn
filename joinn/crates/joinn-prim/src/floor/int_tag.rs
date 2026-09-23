//! Read an ℤ tag from a value.

use joinn_frame::{FrameId, Term, Value};
use num_traits::ToPrimitive;

pub(in crate::floor) fn int_tag(v: &Value) -> Option<i64> {
    if v.frame().id != FrameId::Int {
        return None;
    }
    match v.term() {
        Term::Int(n) => n.to_i64(),
        _ => None,
    }
}
