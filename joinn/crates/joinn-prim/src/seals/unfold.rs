//! Restore a sealed native allele.

use joinn_dna::{AlleleBody, Cell, NativeId};

/// Restore a sealed native allele.
pub fn unfold(cell: &Cell, sealed: NativeId) -> Cell {
    let mut c = cell.clone();
    if let Some(a) = c.alleles.first_mut() {
        a.body = AlleleBody::Native(sealed);
    }
    c
}
