//! Every codon.

use super::Codon;

impl Codon {
    /// All 64, in bit order: `AAA` first, `CCC` last.
    pub fn all() -> Vec<Codon> {
        (0..64).map(Codon).collect()
    }
}
