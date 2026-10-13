//! The codon on a segment.

use joinn_codex::Codon;

use super::Chromosome;

impl Chromosome {
    /// `None` for a silent segment: the reader returns nothing for it.
    pub fn codon_at(&self, segment: u64) -> Option<Codon> {
        let k = usize::try_from(segment).ok()?;
        let [a, b, c] = self.strand.letters().get(3 * k..3 * k + 3)? else {
            return None;
        };
        Some(Codon::from_letters([*a, *b, *c]))
    }
}
