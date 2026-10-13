//! Print a chromosome as its file block.

use joinn_strand::{StrandFile, print_strand_file};

use super::Chromosome;

/// The strand file, then `node <d> gene <hash>` per reference, sorted by d.
pub fn print_chromosome(c: &Chromosome) -> String {
    let mut out = print_strand_file(&StrandFile {
        codex: c.codex,
        strand: c.strand.clone(),
    });
    for (d, h) in &c.genes {
        out.push_str(&format!("node {d} gene {}\n", h.to_hex()));
    }
    out
}
