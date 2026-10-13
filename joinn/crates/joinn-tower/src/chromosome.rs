//! A chromosome: a strand of codons placed on a tower. Codon k (from the
//! head, in frame from letter 0) sits on segment k; the order is the smallest
//! n with 4ⁿ − 1 ≥ the codon count, derived, never stored. Segments past the
//! last codon are silent: not stored and not null. Each node may hold one gene
//! reference. Its identity is its strand's hash.

mod codon_at;
mod codons;
mod floor;
mod new;
mod order;
mod parse_chromosomes;
mod print_chromosome;

pub use parse_chromosomes::parse_chromosomes;
pub use print_chromosome::print_chromosome;

use std::collections::BTreeMap;

use joinn_frame::Hash;
use joinn_strand::Strand;

/// A strand on its tower, with its gene references.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Chromosome {
    codex: u32,
    strand: Strand,
    genes: BTreeMap<u64, Hash>,
}

impl Chromosome {
    /// The codex its strand file names.
    pub fn codex(&self) -> u32 {
        self.codex
    }

    /// The strand.
    pub fn strand(&self) -> &Strand {
        &self.strand
    }

    /// Node → the hash of the chromosome it references. Nodes without a
    /// reference are absent.
    pub fn genes(&self) -> &BTreeMap<u64, Hash> {
        &self.genes
    }

    /// The strand's hash.
    pub fn identity(&self) -> Hash {
        self.strand.identity()
    }

    /// Codons stored: the strand's, never the silent segments.
    pub fn stored_codons(&self) -> usize {
        self.strand.len() / 3
    }

    /// 4ⁿ − 1 segments at order n.
    pub fn segments(&self) -> u64 {
        (1u64 << (2 * self.order())) - 1
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use joinn_codex::{AminoAcid, Meaning, codon_of};
    use joinn_frame::Verdict;
    use joinn_strand::{Strand, StrandFile, parse_letters};

    use super::{Chromosome, parse_chromosomes, print_chromosome};

    fn chromosome(letters: &str) -> Chromosome {
        let Verdict::Ok(strand) = parse_letters(letters) else {
            panic!("letters");
        };
        match Chromosome::new(StrandFile { codex: 1, strand }, BTreeMap::new()) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn a_20_codon_strand_gets_order_3_and_stores_20_codons_not_63() {
        let Some(pair) = codon_of(Meaning::Acid(AminoAcid::Pair)) else {
            panic!("pair has a codon");
        };
        let strand = Strand::new((0..20).flat_map(|_| pair.letters()).collect());
        let Verdict::Ok(c) = Chromosome::new(
            StrandFile {
                codex: 1,
                strand: strand.clone(),
            },
            BTreeMap::new(),
        ) else {
            panic!("20 codons place");
        };
        assert_eq!(c.order(), 3);
        assert_eq!(c.segments(), 63);
        assert_eq!(c.stored_codons(), 20);
        assert_eq!(c.codons().len(), 20);
        assert_eq!(
            strand.pack().bytes().len(),
            15,
            "60 letters, four to a byte"
        );
        assert_eq!(c.codon_at(19), Some(pair));
        assert!((20..63).all(|s| c.codon_at(s).is_none()), "silent");
    }

    #[test]
    fn the_empty_strand_is_order_0() {
        let c = chromosome("");
        assert_eq!(c.order(), 0);
        assert_eq!(c.segments(), 0);
        assert_eq!(c.stored_codons(), 0);
        assert_eq!(chromosome("AAT ATT").order(), 1);
        assert_eq!(chromosome("AAT AAA TTT ATT").order(), 2);
    }

    #[test]
    fn floors_keep_the_level_filtered_codons() {
        let c = chromosome("AAT AAA TTT CAC GTG AAC GTT CAA TTG GAA TTC AAG CTT ATT");
        assert_eq!(c.order(), 2);
        let top: Vec<(u64, String)> = c.floor(1).into_iter().map(|(s, k)| (s, k.text())).collect();
        assert_eq!(
            top,
            [
                (3, "CAC".to_owned()),
                (7, "CAA".to_owned()),
                (11, "AAG".to_owned())
            ]
        );
        assert_eq!(c.floor(0).len(), 14);
    }

    #[test]
    fn a_partial_codon_and_a_gene_past_the_last_node_are_refused() {
        let Verdict::Ok(strand) = parse_letters("AAT AT") else {
            panic!("letters");
        };
        match Chromosome::new(StrandFile { codex: 1, strand }, BTreeMap::new()) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "tower: strand of 5 letters ends 2 letters into a codon; acceptance is whole codons"
            ),
            Verdict::Ok(_) => panic!("a partial codon placed"),
        }
        let text = format!(
            "codex 1\nAAT ATT\nnode 4 gene {}\n",
            chromosome("AAT ATT").identity().to_hex()
        );
        match parse_chromosomes(&text) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "tower: node 4 holds a gene, but order 1 has nodes 0 to 3; acceptance is a node of the tower"
            ),
            Verdict::Ok(_) => panic!("node 4 at order 1"),
        }
    }

    #[test]
    fn a_chromosome_file_round_trips() {
        let target = chromosome("AAT ATT").identity().to_hex();
        let text = format!(
            "codex 1\nAAT AAA TTT ATT\nnode 2 gene {target}\nnode 5 gene {target}\n\ncodex 1\nAAT ATT\n"
        );
        let Verdict::Ok(all) = parse_chromosomes(&text) else {
            panic!("reads");
        };
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].genes().len(), 2);
        let printed: Vec<String> = all.iter().map(print_chromosome).collect();
        assert_eq!(printed.join("\n"), text);
        let unsorted =
            format!("codex 1\nAAT AAA TTT ATT\nnode 5 gene {target}\nnode 2 gene {target}\n");
        assert!(parse_chromosomes(&unsorted).is_refused());
    }
}
