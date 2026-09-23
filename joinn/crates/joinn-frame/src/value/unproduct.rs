//! `Value::unproduct`.

use super::Value;
use crate::term::Term;

impl Value {
    /// Inverse of `product`. `None` when this is not a two-part Seq.
    pub fn unproduct(&self) -> Option<(Value, Value)> {
        match &self.term {
            Term::Seq(xs) if xs.len() == 2 => Some((
                Self {
                    frame: self.frame,
                    term: xs[0].clone(),
                },
                Self {
                    frame: self.frame,
                    term: xs[1].clone(),
                },
            )),
            _ => None,
        }
    }
}
