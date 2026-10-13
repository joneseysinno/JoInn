//! A meaning's printed name.

use super::Meaning;

impl Meaning {
    /// `START`, `STOP`, or the amino acid's name.
    pub fn name(self) -> &'static str {
        match self {
            Meaning::Start => "START",
            Meaning::Stop => "STOP",
            Meaning::Acid(a) => a.name(),
        }
    }
}
