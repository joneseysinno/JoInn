//! A letter's two bits.

use super::Letter;

impl Letter {
    /// `0b00` … `0b11`: A, T, G, C.
    pub fn bits(self) -> u8 {
        match self {
            Letter::A => 0b00,
            Letter::T => 0b01,
            Letter::G => 0b10,
            Letter::C => 0b11,
        }
    }
}
