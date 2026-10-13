//! Find a path by which a chromosome reaches itself.

use std::collections::{BTreeMap, BTreeSet};

use joinn_frame::Hash;

use crate::chromosome::Chromosome;

/// The first path `entering → … → entering` through gene references, where
/// `held` are the chromosomes already in the pool and `entering` is the one
/// coming in. References to hashes nobody holds end the path.
pub(crate) fn cycle(held: &BTreeMap<Hash, Chromosome>, entering: &Chromosome) -> Option<Vec<Hash>> {
    let me = entering.identity();
    let genes_of = |h: &Hash| -> Vec<Hash> {
        if *h == me {
            entering.genes().values().copied().collect()
        } else {
            held.get(h)
                .map(|c| c.genes().values().copied().collect())
                .unwrap_or_default()
        }
    };
    let mut stack = vec![vec![me]];
    let mut seen = BTreeSet::new();
    while let Some(path) = stack.pop() {
        let Some(last) = path.last().copied() else {
            continue;
        };
        for next in genes_of(&last).into_iter().rev() {
            let mut longer = path.clone();
            longer.push(next);
            if next == me {
                return Some(longer);
            }
            if seen.insert(next) {
                stack.push(longer);
            }
        }
    }
    None
}
