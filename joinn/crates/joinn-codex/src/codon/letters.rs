//! A codon's three letters.

use joinn_strand::Letter;

use super::Codon;

impl Codon {
    /// Head to tail.
    pub fn letters(self) -> [Letter; 3] {
        [
            Letter::from_bits(self.0 >> 4),
            Letter::from_bits(self.0 >> 2),
            Letter::from_bits(self.0),
        ]
    }
}
