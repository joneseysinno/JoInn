//! Open a node's gene in place.

use joinn_frame::Verdict;
use joinn_tower::{Chromosome, short_hash};

use super::FoldScene;
use crate::refuse::refuse;

impl FoldScene {
    /// `gene` is what the pool gave for `node` (plan 8 §2.7); the scene only
    /// draws it. A gene whose hash is not the node's reference is refused.
    pub fn open(&mut self, node: u64, gene: Chromosome) -> Verdict<()> {
        let Some(named) = self.chromosome.genes().get(&node) else {
            return refuse(format!(
                "fold view: node {node} holds no gene; acceptance is a node with a ring"
            ));
        };
        if *named != gene.identity() {
            return refuse(format!(
                "fold view: node {node} names gene {}, not {}; acceptance is the gene the pool gives for the node",
                short_hash(named),
                short_hash(&gene.identity())
            ));
        }
        let mut opened = self.opened.clone();
        opened.insert(node, gene);
        self.redraw(self.floor, opened)
    }
}
