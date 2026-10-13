//! A chromosome's order.

use super::Chromosome;

impl Chromosome {
    /// The smallest n with 4ⁿ − 1 ≥ the codon count; 0 holds nothing.
    pub fn order(&self) -> u32 {
        let count = self.stored_codons() as u64;
        let mut n = 0;
        while (1u64 << (2 * n)) - 1 < count {
            n += 1;
        }
        n
    }
}
