//! A letter's strength: the high bit.

use super::{Letter, Strength};

impl Letter {
    /// Weak (A, T) or strong (G, C).
    pub fn strength(self) -> Strength {
        if self.bits() & 0b10 == 0 {
            Strength::Weak
        } else {
            Strength::Strong
        }
    }
}
