//! A codon's letters as text.

use super::Codon;

impl Codon {
    /// Three capitals, e.g. `AAT`.
    pub fn text(self) -> String {
        self.letters().iter().map(|l| l.to_char()).collect()
    }
}
