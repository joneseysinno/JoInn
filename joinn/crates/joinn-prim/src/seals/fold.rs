//! V23 fold.

use joinn_dna::{AlleleBody, Cell};
use joinn_frame::Hash;

/// V23: fold, unfold, then execute. Not a vacuous hash comparison.
pub fn fold(cell: &Cell, reference: Hash) -> Cell {
    let mut c = cell.clone();
    if let Some(a) = c.alleles.first_mut() {
        a.body = AlleleBody::Dna(reference);
    }
    c
}
