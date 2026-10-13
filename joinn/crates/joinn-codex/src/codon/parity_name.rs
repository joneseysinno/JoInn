//! A parity's printed name.

use super::Parity;

impl Parity {
    /// `even` or `odd`.
    pub fn name(self) -> &'static str {
        match self {
            Parity::Even => "even",
            Parity::Odd => "odd",
        }
    }
}
