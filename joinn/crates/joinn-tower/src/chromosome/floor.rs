//! A chromosome zoomed out to a floor.

use joinn_codex::Codon;

use super::Chromosome;
use crate::level::level;

impl Chromosome {
    /// `(segment, codon)` for the codons on segments of level ≥ `floor`, in
    /// order. Silent segments hold nothing, so they never appear.
    pub fn floor(&self, floor: u32) -> Vec<(u64, Codon)> {
        self.codons()
            .into_iter()
            .enumerate()
            .map(|(k, c)| (k as u64, c))
            .filter(|(s, _)| level(*s) >= floor)
            .collect()
    }
}
