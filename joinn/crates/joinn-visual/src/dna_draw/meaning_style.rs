//! A codon's tint.

use joinn_codex::{AminoAcid, Meaning};

use crate::tables::{STYLE_ACID_FIRST, STYLE_CAP, STYLE_RESERVED};

/// An amino acid's own tint, in table order; START and STOP share theirs; a
/// reserved codon is grey.
pub(crate) fn meaning_style(meaning: Option<Meaning>) -> u32 {
    match meaning {
        None => STYLE_RESERVED,
        Some(Meaning::Start | Meaning::Stop) => STYLE_CAP,
        Some(Meaning::Acid(a)) => (0u32..)
            .zip(AminoAcid::ALL)
            .find(|(_, b)| *b == a)
            .map_or(STYLE_RESERVED, |(i, _)| STYLE_ACID_FIRST + i),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use joinn_codex::{AminoAcid, Meaning};

    use super::meaning_style;
    use crate::tables::{STYLE_ACID_FIRST, STYLE_CAP, STYLE_RESERVED};

    #[test]
    fn twelve_acids_twelve_tints_and_none_is_grey() {
        let tints: BTreeSet<u32> = AminoAcid::ALL
            .iter()
            .map(|a| meaning_style(Some(Meaning::Acid(*a))))
            .collect();
        assert_eq!(tints, (STYLE_ACID_FIRST..STYLE_ACID_FIRST + 12).collect());
        assert_eq!(meaning_style(Some(Meaning::Stop)), STYLE_CAP);
        assert_eq!(meaning_style(None), STYLE_RESERVED);
    }
}
