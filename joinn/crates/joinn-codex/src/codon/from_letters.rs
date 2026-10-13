//! The codon three letters make.

use joinn_strand::Letter;

use super::Codon;

impl Codon {
    /// First letter in the high bits.
    pub fn from_letters(letters: [Letter; 3]) -> Codon {
        let [a, b, c] = letters;
        Codon((a.bits() << 4) | (b.bits() << 2) | c.bits())
    }
}
