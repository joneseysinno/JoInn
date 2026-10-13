//! A side's printed name.

use super::Side;

impl Side {
    /// `sense` or `partner`.
    pub fn name(self) -> &'static str {
        match self {
            Side::Sense => "sense",
            Side::Partner => "partner",
        }
    }
}
