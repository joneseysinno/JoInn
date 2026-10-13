//! A chromosome's codons.

use joinn_codex::Codon;

use super::Chromosome;

impl Chromosome {
    /// In frame from letter 0, head first; codon k sits on segment k.
    pub fn codons(&self) -> Vec<Codon> {
        self.strand
            .letters()
            .chunks_exact(3)
            .filter_map(|c| match c {
                [a, b, c] => Some(Codon::from_letters([*a, *b, *c])),
                _ => None,
            })
            .collect()
    }
}
