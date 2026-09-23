//! Extract bigint from ℤ value.

use joinn_frame::{FrameId, Term, Value};
use num_bigint::BigInt;

pub(crate) fn int_of(v: &Value) -> Option<BigInt> {
    if v.frame().id != FrameId::Int {
        return None;
    }
    match v.term() {
        Term::Int(n) => Some(n.clone()),
        _ => None,
    }
}
