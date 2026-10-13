//! The letter two bits name.

use super::Letter;

impl Letter {
    /// The letter of the low two bits of `bits`; higher bits are not read.
    pub fn from_bits(bits: u8) -> Letter {
        match bits & 0b11 {
            0b00 => Letter::A,
            0b01 => Letter::T,
            0b10 => Letter::G,
            _ => Letter::C,
        }
    }
}
