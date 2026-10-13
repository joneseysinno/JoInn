//! The fold view (plan 8 §2.10): a chromosome on its Hilbert curve. Each
//! segment is tinted by what the reader says its codon means; silent segments
//! are thin and grey; a node holding a gene reference wears a ring. Floor `f`
//! draws exactly the segments the level filter keeps, on the order `f` floors
//! down, scaled up so the picture keeps its size. An opened gene is drawn
//! small beside its node. All integer.

mod fit;
mod gene_pieces;
mod new;
mod open;
mod pieces;
mod print_id;
mod redraw;
mod set_floor;
mod shapes;
mod take_pending;

use std::collections::{BTreeMap, BTreeSet};

use joinn_tower::Chromosome;

use crate::tables::{RowWrite, Tables};

/// Sixteenths from one node to the next at floor 0.
pub const NODE_PITCH: i64 = 64;
/// Sixteenths of margin around the picture.
const MARGIN: i64 = 48;
/// Half-width of a segment holding a codon.
const SEGMENT_HALF: i64 = 8;
/// Half-width of a silent segment.
const SILENT_HALF: i64 = 2;
/// A ring's radius, in sixteenths.
const RING_RADIUS: i64 = 18;
/// The most sixteenths an opened gene's curve spans.
const GENE_SPAN: i64 = 40;

/// A chromosome folded on its tower, drawn as tables.
#[derive(Clone, Debug)]
pub struct FoldScene {
    chromosome: Chromosome,
    floor: u32,
    opened: BTreeMap<u64, Chromosome>,
    tables: Tables,
    pending: BTreeSet<RowWrite>,
}

impl FoldScene {
    /// The tables as they stand.
    pub fn tables(&self) -> &Tables {
        &self.tables
    }

    /// The chromosome drawn.
    pub fn chromosome(&self) -> &Chromosome {
        &self.chromosome
    }

    /// The floor drawn: 0 is every segment.
    pub fn floor(&self) -> u32 {
        self.floor
    }

    /// The genes opened in place, by node.
    pub fn opened(&self) -> &BTreeMap<u64, Chromosome> {
        &self.opened
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use joinn_frame::Verdict;
    use joinn_strand::{StrandFile, parse_letters, parse_strand_file};
    use joinn_tower::{Chromosome, level_filter, parse_chromosomes};

    use super::FoldScene;
    use crate::camera::FitCamera;
    use crate::pick::{CODON_TAG, Pick, Shape, cpu_pick};
    use crate::strand_scene::StrandScene;

    fn twelve() -> Chromosome {
        match parse_chromosomes(include_str!(
            "../../../corpus/phase8/tower/twelve.chromosome"
        )) {
            Verdict::Ok(mut c) if c.len() == 1 => c.remove(0),
            Verdict::Ok(_) => panic!("one chromosome"),
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    fn scene(c: Chromosome) -> FoldScene {
        match FoldScene::new(c) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    /// Segment index → part (0 a codon, 1 silent) of every segment stroke.
    fn drawn(s: &FoldScene) -> BTreeMap<u64, u32> {
        s.tables()
            .stroke
            .iter()
            .filter(|r| r.owner[1] < 2 && r.owner[2] & CODON_TAG == CODON_TAG)
            .map(|r| (u64::from(r.owner[2] & !CODON_TAG), r.owner[1]))
            .collect()
    }

    fn segment_strokes(s: &FoldScene) -> usize {
        s.tables()
            .stroke
            .iter()
            .filter(|r| r.owner[1] < 2 && r.owner[2] & CODON_TAG == CODON_TAG)
            .count()
    }

    #[test]
    fn segments_drawn_are_the_codons_and_the_silent_ones() {
        let twelve = scene(twelve());
        assert_eq!(segment_strokes(&twelve), 15);
        let parts = drawn(&twelve);
        assert_eq!(parts.values().filter(|p| **p == 0).count(), 14);
        assert_eq!(parts.values().filter(|p| **p == 1).count(), 1);
        let Verdict::Ok(strand) = parse_letters(&"AAA".repeat(20)) else {
            panic!("letters");
        };
        let twenty = match Chromosome::new(StrandFile { codex: 1, strand }, BTreeMap::new()) {
            Verdict::Ok(c) => scene(c),
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let parts = drawn(&twenty);
        assert_eq!(
            segment_strokes(&twenty),
            63,
            "order 3: 20 codons + 43 silent"
        );
        assert_eq!(parts.values().filter(|p| **p == 0).count(), 20);
        assert_eq!(parts.values().filter(|p| **p == 1).count(), 43);
    }

    #[test]
    fn each_floor_draws_exactly_the_level_filtered_segments() {
        let c = twelve();
        let all: Vec<u64> = (0..c.segments()).collect();
        let mut s = scene(c.clone());
        for floor in 0..=c.order() {
            assert_eq!(s.set_floor(floor), Verdict::Ok(()), "floor {floor}");
            let parts = drawn(&s);
            let kept: BTreeSet<u64> = level_filter(&all, floor).into_iter().collect();
            assert_eq!(
                parts.keys().copied().collect::<BTreeSet<_>>(),
                kept,
                "floor {floor}"
            );
            assert_eq!(segment_strokes(&s), kept.len(), "floor {floor}");
            let codons: BTreeSet<u64> = c.floor(floor).into_iter().map(|(seg, _)| seg).collect();
            let tinted: BTreeSet<u64> = parts
                .iter()
                .filter(|(_, p)| **p == 0)
                .map(|(seg, _)| *seg)
                .collect();
            assert_eq!(tinted, codons, "floor {floor}: the tower's floor");
            assert!(s.take_pending().is_some(), "floor {floor} writes rows");
        }
        match s.set_floor(c.order() + 1) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "fold view: floor 3 is above order 2's top floor; acceptance is a floor from 0 to 2"
            ),
            Verdict::Ok(()) => panic!("floor 3 drew"),
        }
        assert_eq!(s.floor(), 2, "a refusal changes nothing");
    }

    #[test]
    fn picking_codon_7_in_either_view_returns_the_same_address() {
        let c = twelve();
        let strand = match StrandScene::new(c.strand(), &c.strand().partner()) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let fold = scene(c.clone());
        let named = |shapes: &[Shape],
                     camera: &FitCamera,
                     print: &dyn Fn([u32; 4]) -> String|
         -> BTreeSet<String> {
            let picked = match cpu_pick(shapes, camera) {
                Verdict::Ok(p) => p,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            picked
                .pixels
                .iter()
                .filter_map(|p| match p {
                    Pick::Owned(id) if id[2] == CODON_TAG | 7 && id[1] == 0 => Some(print(*id)),
                    _ => None,
                })
                .collect()
        };
        let in_strand = named(&strand.shapes(), &strand.fit(640, 360), &|id| {
            strand.print_id(id)
        });
        let in_fold = named(&fold.shapes(), &fold.fit(640, 360), &|id| fold.print_id(id));
        let address = format!("{}.codon7", joinn_tower::short_hash(&c.identity()));
        assert_eq!(in_strand, BTreeSet::from([address.clone()]));
        assert_eq!(in_fold, BTreeSet::from([address]));
    }

    #[test]
    fn a_ring_names_its_gene_and_an_opened_gene_draws_beside_it() {
        let c = twelve();
        let mut s = scene(c.clone());
        let rings: BTreeSet<u32> = s
            .tables()
            .stroke
            .iter()
            .filter(|r| r.owner[1] == 2)
            .map(|r| r.owner[2] & !CODON_TAG)
            .collect();
        assert_eq!(rings, BTreeSet::from([5, 10]));
        assert_eq!(
            s.print_id([1, 2, CODON_TAG | 5, 1]),
            "edf1529a6fc2.node5 gene 8b126fce9ab6"
        );
        let pair_split = match parse_strand_file(include_str!(
            "../../../corpus/phase8/strands/pair-split.strand"
        )) {
            Verdict::Ok(f) => match Chromosome::new(f, BTreeMap::new()) {
                Verdict::Ok(g) => g,
                Verdict::Refused(r) => panic!("{}", r.reason),
            },
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        match s.open(10, pair_split.clone()) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "fold view: node 10 names gene 994c4e6ed409, not 8b126fce9ab6; acceptance is the gene the pool gives for the node"
            ),
            Verdict::Ok(()) => panic!("the wrong gene opened"),
        }
        assert_eq!(s.open(5, pair_split), Verdict::Ok(()));
        assert!(s.tables().stroke.iter().any(|r| r.owner[1] == 3));
        assert_eq!(
            s.print_id([1, 3, CODON_TAG | 5, 1]),
            "gene 8b126fce9ab6 at edf1529a6fc2.node5"
        );
        assert_eq!(s.print_id([1, 0, 0, 1]), "fold edf1529a6fc2 floor 0");
    }
}
