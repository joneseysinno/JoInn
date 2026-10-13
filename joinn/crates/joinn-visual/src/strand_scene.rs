//! The strand view (plan 8 §2.10): a double helix seen from a little above,
//! one rung per letter pair, rungs in threes, each codon tinted by what the
//! reader says it means, START and STOP closed by end caps, every name in the
//! stroke font under its codon, and a glow on every pair whose partner letter
//! is wrong. The helix is still: a turn would owe a tick with no input (V147).
//! All integer: the turn is a 12-step table of sines in thousandths.

mod fit;
mod helix;
mod new;
mod pieces;
mod print_id;
mod shapes;
mod take_pending;

use std::collections::BTreeSet;

use joinn_frame::Hash;

use crate::tables::{RowWrite, Tables};

/// Sixteenths from one letter's rung to the next.
pub const LETTER_PITCH: i64 = 32;
/// Extra sixteenths between one codon and the next.
pub const CODON_GAP: i64 = 16;
/// The helix's radius, in sixteenths.
pub const HELIX_RADIUS: i64 = 64;
/// Sixteenths of margin around the picture.
const MARGIN: i64 = 48;
/// The helix's axis, in sixteenths from the top.
const AXIS: i64 = MARGIN + HELIX_RADIUS + 16;
/// sin(30° × j) in thousandths: one turn every twelve letters.
const SINES: [i64; 12] = [
    0, 500, 866, 1000, 866, 500, 0, -500, -866, -1000, -866, -500,
];

/// A strand and its partner drawn as tables.
#[derive(Clone, Debug)]
pub struct StrandScene {
    hash: Hash,
    codons: usize,
    tables: Tables,
    pending: BTreeSet<RowWrite>,
}

impl StrandScene {
    /// The tables as they stand.
    pub fn tables(&self) -> &Tables {
        &self.tables
    }

    /// The strand's hash: the head of every address it prints.
    pub fn hash(&self) -> Hash {
        self.hash
    }

    /// Whole codons drawn.
    pub fn codons(&self) -> usize {
        self.codons
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use joinn_frame::Verdict;
    use joinn_strand::{Letter, Strand, parse_strand_file};

    use super::StrandScene;
    use crate::pick::{CODON_TAG, Pick, cpu_pick};
    use crate::tables::{
        STYLE_ACID_FIRST, STYLE_CAP, STYLE_END_CAP, STYLE_GLOW, STYLE_LETTER_A, STYLE_RESERVED,
    };

    fn twelve() -> Strand {
        match parse_strand_file(include_str!("../../../corpus/phase8/strands/twelve.strand")) {
            Verdict::Ok(f) => f.strand,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    fn scene(strand: &Strand, partner: &Strand) -> StrandScene {
        match StrandScene::new(strand, partner) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    /// Strokes per codon index whose style satisfies `keep`.
    fn per_codon(s: &StrandScene, keep: impl Fn(u32) -> bool) -> BTreeMap<u32, usize> {
        let mut out = BTreeMap::new();
        for row in s.tables().stroke.iter().filter(|r| keep(r.style)) {
            if row.owner[2] & CODON_TAG == CODON_TAG {
                *out.entry(row.owner[2] & !CODON_TAG).or_insert(0) += 1;
            }
        }
        out
    }

    fn letter(style: u32) -> bool {
        (STYLE_LETTER_A..STYLE_LETTER_A + 4).contains(&style)
    }

    fn tint(style: u32) -> bool {
        (STYLE_ACID_FIRST..STYLE_ACID_FIRST + 12).contains(&style)
            || style == STYLE_CAP
            || style == STYLE_RESERVED
    }

    fn glows(s: &StrandScene) -> usize {
        s.tables()
            .stroke
            .iter()
            .filter(|r| r.style == STYLE_GLOW)
            .count()
    }

    #[test]
    fn three_rungs_a_side_per_codon_one_tint_each_and_one_glow_per_flipped_letter() {
        let strand = twelve();
        let s = scene(&strand, &strand.partner());
        assert_eq!(s.codons(), 14);
        let rungs = per_codon(&s, letter);
        assert_eq!(rungs.len(), 14);
        assert!(
            rungs.values().all(|n| *n == 2 * 3),
            "three rungs on each side of every codon: {rungs:?}"
        );
        let tints = per_codon(&s, tint);
        assert_eq!(tints.len(), 14);
        assert!(
            tints.values().all(|n| *n == 1),
            "one tint per codon: {tints:?}"
        );
        assert_eq!(
            s.tables()
                .stroke
                .iter()
                .filter(|r| r.style == STYLE_END_CAP)
                .count(),
            2,
            "START and STOP each closed by a cap"
        );
        assert_eq!(glows(&s), 0, "the reader's own partner never glows");
        for at in [0, 20, 41] {
            let Some(was) = strand.partner().letters().get(at).copied() else {
                panic!("letter {at}");
            };
            let flipped = Letter::ALL
                .into_iter()
                .find(|l| *l != was)
                .and_then(|l| strand.partner().with_letter(at, l));
            let Some(flipped) = flipped else {
                panic!("flip {at}");
            };
            assert_eq!(
                glows(&scene(&strand, &flipped)),
                1,
                "flipped partner letter {at}"
            );
        }
    }

    #[test]
    fn a_partner_of_another_length_is_refused() {
        let strand = twelve();
        let short = Strand::new(strand.letters()[1..].to_vec());
        match StrandScene::new(&strand, &short) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "strand view: a partner of 41 letters beside a strand of 42; acceptance is a partner as long as its strand"
            ),
            Verdict::Ok(_) => panic!("a short partner drew"),
        }
    }

    #[test]
    fn every_codon_owns_pixels_and_prints_its_address() {
        let strand = twelve();
        let s = scene(&strand, &strand.partner());
        let picked = match cpu_pick(&s.shapes(), &s.fit(640, 360)) {
            Verdict::Ok(p) => p,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let codons: BTreeSet<u32> = picked
            .pixels
            .iter()
            .filter_map(|p| match p {
                Pick::Owned(id) if id[2] & CODON_TAG == CODON_TAG => Some(id[2] & !CODON_TAG),
                _ => None,
            })
            .collect();
        assert_eq!(codons, (0..14).collect());
        let head = joinn_tower::short_hash(&strand.identity());
        assert_eq!(
            s.print_id([1, 0, CODON_TAG | 7, 1]),
            format!("{head}.codon7")
        );
        assert_eq!(s.print_id([1, 0, 0, 1]), format!("strand {head}"));
        assert_eq!(s.print_id([0; 4]), "background");
    }
}
