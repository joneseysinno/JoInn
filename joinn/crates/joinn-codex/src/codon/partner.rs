//! The partner codon.

use super::Codon;

impl Codon {
    /// The three letters' partners, reversed. Each letter's low bit flips, so
    /// three bits flip and the parity always changes.
    pub fn partner(self) -> Codon {
        let [a, b, c] = self.letters();
        Codon::from_letters([c.partner(), b.partner(), a.partner()])
    }
}
