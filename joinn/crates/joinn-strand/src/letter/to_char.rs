//! A letter's name.

use super::Letter;

impl Letter {
    /// `A`, `T`, `G` or `C`: the only name a letter has.
    pub fn to_char(self) -> char {
        match self {
            Letter::A => 'A',
            Letter::T => 'T',
            Letter::G => 'G',
            Letter::C => 'C',
        }
    }
}
