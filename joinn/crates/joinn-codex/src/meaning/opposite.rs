//! A meaning's opposite.

use super::Meaning;

impl Meaning {
    /// START and STOP are each other's; an amino acid's is its row partner.
    pub fn opposite(self) -> Meaning {
        match self {
            Meaning::Start => Meaning::Stop,
            Meaning::Stop => Meaning::Start,
            Meaning::Acid(a) => Meaning::Acid(a.opposite()),
        }
    }
}
