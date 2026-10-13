//! Fold a chromosome.

use std::collections::{BTreeMap, BTreeSet};

use joinn_frame::Verdict;
use joinn_tower::Chromosome;

use super::FoldScene;
use crate::tables::Tables;

impl FoldScene {
    /// Floor 0, nothing opened; every row pending until the first draw.
    pub fn new(chromosome: Chromosome) -> Verdict<FoldScene> {
        let mut scene = FoldScene {
            chromosome,
            floor: 0,
            opened: BTreeMap::new(),
            tables: Tables::empty(),
            pending: BTreeSet::new(),
        };
        scene.redraw(0, BTreeMap::new()).map(|()| scene)
    }
}
