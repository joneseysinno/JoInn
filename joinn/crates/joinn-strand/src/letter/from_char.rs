//! The letter a character names, if any.

use super::Letter;

impl Letter {
    /// `A`, `T`, `G` or `C`; every other character names no letter.
    pub fn from_char(c: char) -> Option<Letter> {
        match c {
            'A' => Some(Letter::A),
            'T' => Some(Letter::T),
            'G' => Some(Letter::G),
            'C' => Some(Letter::C),
            _ => None,
        }
    }
}
