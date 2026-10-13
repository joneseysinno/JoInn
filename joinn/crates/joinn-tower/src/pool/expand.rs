//! Open a node's gene.

use joinn_frame::Verdict;

use super::{Pool, short};
use crate::chromosome::Chromosome;
use crate::refuse::refuse;

impl Pool {
    /// The chromosome `node` of `c` references: one read. A node with no
    /// reference, or a reference the pool does not hold, is refused.
    pub fn expand(&self, c: &Chromosome, node: u64) -> Verdict<&Chromosome> {
        let Some(h) = c.genes().get(&node) else {
            return refuse(format!(
                "pool: node {node} holds no gene; acceptance is a node with a gene reference"
            ));
        };
        self.reads.set(self.reads.get() + 1);
        match self.chromosomes.get(h) {
            Some(found) => Verdict::Ok(found),
            None => refuse(format!(
                "pool: node {node} names gene {}, which the pool does not hold",
                short(h)
            )),
        }
    }
}
