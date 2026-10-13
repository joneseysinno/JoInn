//! What a named codon means: START, STOP, or one of twelve amino acids. Every
//! meaning has an opposite (D19); START and STOP are each other's.

mod acid_name;
mod acid_opposite;
mod name;
mod opposite;

/// The amino acids of codex 1, in table order.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AminoAcid {
    /// `pair` (`AAA`); opposite `split`.
    Pair,
    /// `split` (`TTT`); opposite `pair`.
    Split,
    /// `build` (`CAC`); opposite `case`.
    Build,
    /// `case` (`GTG`); opposite `build`.
    Case,
    /// `refer` (`AAC`); opposite `return`.
    Refer,
    /// `return` (`GTT`); opposite `refer`.
    Return,
    /// `sense` (`CAA`); opposite `show`.
    Sense,
    /// `show` (`TTG`); opposite `sense`.
    Show,
    /// `keep` (`GAA`); opposite `recall`.
    Keep,
    /// `recall` (`TTC`); opposite `keep`.
    Recall,
    /// `grant` (`AAG`); opposite `revoke`.
    Grant,
    /// `revoke` (`CTT`); opposite `grant`.
    Revoke,
}

/// A named codon's meaning.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Meaning {
    /// Opens a protein.
    Start,
    /// Closes a protein.
    Stop,
    /// An amino acid.
    Acid(AminoAcid),
}

impl AminoAcid {
    /// Table order: each row's left, then its right.
    pub const ALL: [AminoAcid; 12] = [
        AminoAcid::Pair,
        AminoAcid::Split,
        AminoAcid::Build,
        AminoAcid::Case,
        AminoAcid::Refer,
        AminoAcid::Return,
        AminoAcid::Sense,
        AminoAcid::Show,
        AminoAcid::Keep,
        AminoAcid::Recall,
        AminoAcid::Grant,
        AminoAcid::Revoke,
    ];
}

#[cfg(test)]
mod tests {
    use super::{AminoAcid, Meaning};

    #[test]
    fn opposite_is_an_involution_with_no_fixed_meaning() {
        let mut all: Vec<Meaning> = AminoAcid::ALL.iter().map(|a| Meaning::Acid(*a)).collect();
        all.extend([Meaning::Start, Meaning::Stop]);
        for m in all {
            assert_ne!(m.opposite(), m, "{}", m.name());
            assert_eq!(m.opposite().opposite(), m, "{}", m.name());
        }
        assert_eq!(Meaning::Start.opposite(), Meaning::Stop);
    }

    #[test]
    fn names_are_the_tables() {
        let names: Vec<&str> = AminoAcid::ALL.iter().map(|a| a.name()).collect();
        assert_eq!(
            names,
            [
                "pair", "split", "build", "case", "refer", "return", "sense", "show", "keep",
                "recall", "grant", "revoke"
            ]
        );
        assert_eq!(Meaning::Start.name(), "START");
        assert_eq!(Meaning::Stop.name(), "STOP");
    }
}
