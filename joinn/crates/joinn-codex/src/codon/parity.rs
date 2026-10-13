//! A codon's parity.

use super::{Codon, Parity};

impl Codon {
    /// The number of 1 bits in the six, mod 2.
    pub fn parity(self) -> Parity {
        if self.0.count_ones() % 2 == 1 {
            Parity::Odd
        } else {
            Parity::Even
        }
    }
}
