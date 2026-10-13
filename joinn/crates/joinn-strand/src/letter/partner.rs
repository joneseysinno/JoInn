//! A letter's partner: the low bit flipped.

use super::Letter;

impl Letter {
    /// A ↔ T, G ↔ C.
    pub fn partner(self) -> Letter {
        Letter::from_bits(self.bits() ^ 0b01)
    }
}
