//! `Value::product`.

use super::Value;
use crate::term::Term;

impl Value {
    /// The only product: two values as one. A pair belongs to no frame; it
    /// wears the left value's tag because `Value` is frame-tagged today (R38).
    pub fn product(left: Value, right: Value) -> Self {
        Self {
            frame: left.frame,
            term: Term::Seq(vec![left.term, right.term]),
        }
    }
}
