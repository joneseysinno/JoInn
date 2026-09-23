//! Closed accessibility roles for this phase (R45).

mod as_str;

use std::fmt;

/// Closed at four so Phase 8 inherits a vocabulary.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Role {
    /// The instance itself.
    Cell,
    /// An in-port.
    Input,
    /// An out-port.
    Output,
    /// A membrane refusal, shown as a thing.
    Refusal,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
