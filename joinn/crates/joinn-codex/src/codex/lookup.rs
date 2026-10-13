//! What a codon means in codex 1.

use super::CODEX_1;
use crate::codon::Codon;
use crate::meaning::Meaning;

/// Its meaning, or `None` for a reserved codon.
pub fn lookup(codon: Codon) -> Option<Meaning> {
    CODEX_1.iter().find_map(|row| {
        if Codon::from_letters(row.codon) == codon {
            Some(row.means)
        } else if Codon::from_letters(row.partner) == codon {
            Some(row.partner_means)
        } else {
            None
        }
    })
}
