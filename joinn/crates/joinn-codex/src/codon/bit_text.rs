//! A codon's bits as text.

use super::Codon;

impl Codon {
    /// Two bits per letter, letters separated by a space: `AAT` is `00 00 01`.
    pub fn bit_text(self) -> String {
        let b = format!("{:06b}", self.0);
        format!("{} {} {}", &b[0..2], &b[2..4], &b[4..6])
    }
}
