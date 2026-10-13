//! Enter in the fold: open a ring's gene in place.

use joinn_frame::Verdict;
use joinn_tower::short_hash;

use super::Dna;

impl Dna {
    /// The selected ring, or else the first ring not yet open: one read of the
    /// pool, then `opened node <d>: gene <hash>, <n> codons (pool reads <r>)`.
    /// A refusal prints and nothing opens. Nothing in the strand view.
    pub fn open_gene(&mut self) -> Vec<String> {
        if !self.folded {
            return Vec::new();
        }
        let chromosome = self.fold.chromosome();
        let node = self.selected.or_else(|| {
            chromosome
                .genes()
                .keys()
                .find(|d| !self.fold.opened().contains_key(d))
                .copied()
        });
        let Some(node) = node else {
            return vec!["  (every ring is open: nothing to read)".to_owned()];
        };
        let gene = match self.pool.expand(chromosome, node) {
            Verdict::Ok(g) => g.clone(),
            Verdict::Refused(r) => return vec![format!("refused: {}", r.reason)],
        };
        let line = format!(
            "opened node {node}: gene {}, {} codons (pool reads {})",
            short_hash(&gene.identity()),
            gene.stored_codons(),
            self.pool.reads()
        );
        match self.fold.open(node, gene) {
            Verdict::Ok(()) => vec![line],
            Verdict::Refused(r) => vec![format!("refused: {}", r.reason)],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::View;
    use super::super::dna_fixture::twelve;

    #[test]
    fn enter_opens_each_ring_once_reading_the_pool_once_each() {
        let mut d = twelve(true);
        assert_eq!(d.pool.reads(), 0, "opening the view reads nothing");
        assert_eq!(
            d.enter(),
            ["opened node 5: gene 8b126fce9ab6, 4 codons (pool reads 1)"]
        );
        assert_eq!(
            d.enter(),
            ["opened node 10: gene 994c4e6ed409, 2 codons (pool reads 2)"]
        );
        assert_eq!(d.enter(), ["  (every ring is open: nothing to read)"]);
        assert_eq!(d.pool.reads(), 2);
        assert!(d.tables().stroke.iter().any(|r| r.owner[1] == 3));
        assert!(
            twelve(false).enter().is_empty(),
            "the strand view opens nothing"
        );
    }
}
