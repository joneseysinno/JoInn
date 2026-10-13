//! Read a strand's side.

use joinn_strand::Strand;

use super::Side;
use crate::codon::{Codon, Parity};

/// From the head codon's parity; `None` for a strand shorter than a codon.
pub fn side(strand: &Strand) -> Option<Side> {
    let [a, b, c, ..] = strand.letters() else {
        return None;
    };
    Some(match Codon::from_letters([*a, *b, *c]).parity() {
        Parity::Odd => Side::Sense,
        Parity::Even => Side::Partner,
    })
}
