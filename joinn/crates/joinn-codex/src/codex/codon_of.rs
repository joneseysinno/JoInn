//! The codon codex 1 gives a meaning.

use super::CODEX_1;
use crate::codon::Codon;
use crate::meaning::Meaning;

/// The one codon that means `meaning`; every meaning has one in codex 1.
pub fn codon_of(meaning: Meaning) -> Option<Codon> {
    CODEX_1.iter().find_map(|row| {
        if row.means == meaning {
            Some(Codon::from_letters(row.codon))
        } else if row.partner_means == meaning {
            Some(Codon::from_letters(row.partner))
        } else {
            None
        }
    })
}
