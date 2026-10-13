//! Codex 1, exactly Part VIII §6.1. Seven rows of a codon and its partner;
//! the other 50 codons are reserved, and reading one is a refusal, never a
//! guess (D18).

mod check_table;
mod codon_of;
mod lookup;

pub use check_table::check_table;
pub use codon_of::codon_of;
pub use lookup::lookup;

use joinn_strand::Letter;

use crate::meaning::{AminoAcid, Meaning};

/// The codex a strand file names on its first line.
pub const CODEX: u32 = 1;

/// The reader's own version. A separate track from the codex: changing it
/// never changes what a codex-1 strand means.
pub const READER_VERSION: &str = "reader 8.0";

/// One table row: a codon, its meaning, its partner codon, and that one's.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Row {
    /// The left codon.
    pub codon: [Letter; 3],
    /// What it means.
    pub means: Meaning,
    /// The right codon: the left one's partner.
    pub partner: [Letter; 3],
    /// What it means: the opposite.
    pub partner_means: Meaning,
}

use Letter::{A, C, G, T};

/// Codex 1.
pub const CODEX_1: [Row; 7] = [
    Row {
        codon: [A, A, T],
        means: Meaning::Start,
        partner: [A, T, T],
        partner_means: Meaning::Stop,
    },
    Row {
        codon: [A, A, A],
        means: Meaning::Acid(AminoAcid::Pair),
        partner: [T, T, T],
        partner_means: Meaning::Acid(AminoAcid::Split),
    },
    Row {
        codon: [C, A, C],
        means: Meaning::Acid(AminoAcid::Build),
        partner: [G, T, G],
        partner_means: Meaning::Acid(AminoAcid::Case),
    },
    Row {
        codon: [A, A, C],
        means: Meaning::Acid(AminoAcid::Refer),
        partner: [G, T, T],
        partner_means: Meaning::Acid(AminoAcid::Return),
    },
    Row {
        codon: [C, A, A],
        means: Meaning::Acid(AminoAcid::Sense),
        partner: [T, T, G],
        partner_means: Meaning::Acid(AminoAcid::Show),
    },
    Row {
        codon: [G, A, A],
        means: Meaning::Acid(AminoAcid::Keep),
        partner: [T, T, C],
        partner_means: Meaning::Acid(AminoAcid::Recall),
    },
    Row {
        codon: [A, A, G],
        means: Meaning::Acid(AminoAcid::Grant),
        partner: [C, T, T],
        partner_means: Meaning::Acid(AminoAcid::Revoke),
    },
];

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_strand::Letter;

    use super::{CODEX_1, check_table, codon_of, lookup};
    use crate::codon::Codon;
    use crate::meaning::{AminoAcid, Meaning};

    #[test]
    fn the_table_pairs_and_names_14() {
        match check_table(&CODEX_1) {
            Verdict::Ok(named) => assert_eq!(named, 14),
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn every_partner_names_the_opposite_and_reserved_stays_reserved() {
        let mut named = 0;
        for c in Codon::all() {
            match (lookup(c), lookup(c.partner())) {
                (Some(m), Some(p)) => {
                    named += 1;
                    assert_eq!(p, m.opposite(), "{}", c.text());
                }
                (None, None) => {}
                (a, b) => panic!("{} is {a:?} but its partner is {b:?}", c.text()),
            }
        }
        assert_eq!(named, 14);
    }

    #[test]
    fn every_meaning_has_one_codon() {
        let mut all: Vec<Meaning> = AminoAcid::ALL.iter().map(|a| Meaning::Acid(*a)).collect();
        all.extend([Meaning::Start, Meaning::Stop]);
        for m in all {
            assert_eq!(codon_of(m).and_then(lookup), Some(m), "{}", m.name());
        }
        assert_eq!(
            codon_of(Meaning::Start).map(|c| c.text()).as_deref(),
            Some("AAT")
        );
        assert_eq!(
            codon_of(Meaning::Stop).map(|c| c.text()).as_deref(),
            Some("ATT")
        );
    }

    #[test]
    fn aaa_meaning_split_is_refused() {
        let mut rows = CODEX_1;
        rows[1].means = Meaning::Acid(AminoAcid::Split);
        match check_table(&rows) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "codex: AAA means split and its partner TTT means split; acceptance is TTT meaning the opposite, pair"
            ),
            Verdict::Ok(_) => panic!("AAA meaning split was accepted"),
        }
    }

    #[test]
    fn a_listed_partner_that_isnt_one_is_refused() {
        let mut rows = CODEX_1;
        rows[2].partner = [Letter::G, Letter::T, Letter::C];
        match check_table(&rows) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "codex: row CAC lists partner GTC; acceptance is GTG, CAC's partner"
            ),
            Verdict::Ok(_) => panic!("a wrong partner was accepted"),
        }
    }
}
