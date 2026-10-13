//! The gene pool (plan 8 §2.7): chromosomes keyed by their hash, added only
//! through [`Pool::insert`]. A chromosome that reaches itself through gene
//! references is refused when it enters. Nothing is read from the pool until
//! a node is expanded, and every expansion is counted.

mod cycle;
mod expand;
mod insert;
mod short;

pub(crate) use cycle::cycle;
pub use short::short;

use std::cell::Cell;
use std::collections::BTreeMap;

use joinn_frame::Hash;

use crate::chromosome::Chromosome;

/// Chromosomes by hash, and the count of reads so far.
#[derive(Debug, Default)]
pub struct Pool {
    chromosomes: BTreeMap<Hash, Chromosome>,
    reads: Cell<u64>,
}

impl Pool {
    /// Reads so far: one per expansion.
    pub fn reads(&self) -> u64 {
        self.reads.get()
    }

    /// Chromosomes held.
    pub fn len(&self) -> usize {
        self.chromosomes.len()
    }

    /// True when nothing is held.
    pub fn is_empty(&self) -> bool {
        self.chromosomes.is_empty()
    }

    /// The hashes held, in order. Not a read.
    pub fn hashes(&self) -> Vec<Hash> {
        self.chromosomes.keys().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use joinn_frame::{Hash, Verdict};
    use joinn_strand::{StrandFile, parse_letters};

    use super::Pool;
    use crate::chromosome::Chromosome;

    fn chromosome(letters: &str, genes: &[(u64, Hash)]) -> Chromosome {
        let Verdict::Ok(strand) = parse_letters(letters) else {
            panic!("letters");
        };
        match Chromosome::new(
            StrandFile { codex: 1, strand },
            genes.iter().copied().collect::<BTreeMap<_, _>>(),
        ) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn an_unexpanded_chromosome_reads_nothing_and_each_expansion_reads_once() {
        let leaf = chromosome("AAT ATT", &[]);
        let root = chromosome("AAT AAA TTT ATT", &[(5, leaf.identity())]);
        let mut pool = Pool::default();
        assert!(pool.insert(leaf.clone()).is_ok());
        assert!(pool.insert(root.clone()).is_ok());
        assert_eq!(root.floor(0).len(), 4);
        assert_eq!(root.genes().len(), 1);
        assert_eq!(
            pool.reads(),
            0,
            "placing, floors and gene lists read nothing"
        );
        match pool.expand(&root, 5) {
            Verdict::Ok(got) => assert_eq!(got, &leaf),
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
        assert_eq!(pool.reads(), 1);
        assert!(pool.expand(&root, 4).is_refused(), "node 4 holds no gene");
    }

    #[test]
    fn two_chromosomes_naming_each_other_are_refused_on_entry() {
        let a0 = chromosome("AAT AAA TTT ATT", &[]);
        let b = chromosome("AAT CAC GTG ATT", &[(2, a0.identity())]);
        let a = chromosome("AAT AAA TTT ATT", &[(1, b.identity())]);
        let mut pool = Pool::default();
        assert!(pool.insert(a.clone()).is_ok());
        match pool.insert(b.clone()) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                format!(
                    "pool: gene {} contains itself through {} → {} → {}; acceptance is a pool without cycles",
                    super::short(&b.identity()),
                    super::short(&b.identity()),
                    super::short(&a.identity()),
                    super::short(&b.identity())
                )
            ),
            Verdict::Ok(_) => panic!("a cycle entered the pool"),
        }
        assert_eq!(pool.len(), 1);
    }

    #[test]
    fn a_gene_naming_itself_is_refused_and_a_missing_gene_is_refused_at_expansion() {
        let plain = chromosome("AAT ATT", &[]);
        let own = chromosome("AAT ATT", &[(0, plain.identity())]);
        assert!(Pool::default().insert(own).is_refused());
        let ghost = chromosome("AAT CAC GTG ATT", &[]);
        let root = chromosome("AAT AAA TTT ATT", &[(5, ghost.identity())]);
        let mut pool = Pool::default();
        assert!(pool.insert(root.clone()).is_ok());
        match pool.expand(&root, 5) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                format!(
                    "pool: node 5 names gene {}, which the pool does not hold",
                    super::short(&ghost.identity())
                )
            ),
            Verdict::Ok(_) => panic!("a missing gene expanded"),
        }
    }
}
