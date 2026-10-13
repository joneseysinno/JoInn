//! Place a strand on its tower.

use std::collections::BTreeMap;

use joinn_frame::{Hash, Verdict};
use joinn_strand::StrandFile;

use super::Chromosome;
use crate::refuse::refuse;

impl Chromosome {
    /// Whole codons only; every gene reference on a node of the tower.
    pub fn new(file: StrandFile, genes: BTreeMap<u64, Hash>) -> Verdict<Chromosome> {
        let len = file.strand.len();
        if !len.is_multiple_of(3) {
            return refuse(format!(
                "tower: strand of {len} letters ends {} letters into a codon; acceptance is whole codons",
                len % 3
            ));
        }
        let c = Chromosome {
            codex: file.codex,
            strand: file.strand,
            genes,
        };
        let nodes = 1u64 << (2 * c.order());
        if let Some(node) = c.genes.keys().find(|d| **d >= nodes) {
            return refuse(format!(
                "tower: node {node} holds a gene, but order {} has nodes 0 to {}; acceptance is a node of the tower",
                c.order(),
                nodes - 1
            ));
        }
        Verdict::Ok(c)
    }
}
