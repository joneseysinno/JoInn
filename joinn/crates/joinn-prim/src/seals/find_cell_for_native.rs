//! Find the founding cell for a sealed native.

use joinn_dna::{AlleleBody, Cell, NativeId};
use joinn_frame::{Hash, Verdict};
use std::collections::BTreeMap;

use super::refuse_seal::refuse_seal;

/// The unique founding cell whose first allele is `Native(sealed)`.
pub fn find_cell_for_native(cells: &BTreeMap<Hash, Cell>, sealed: &NativeId) -> Verdict<Hash> {
    let mut found = Vec::new();
    for (h, c) in cells {
        if c.coding.lineage.is_some() {
            continue;
        }
        match c.alleles.first().map(|a| &a.body) {
            Some(AlleleBody::Native(id)) if id.0 == sealed.0 => found.push(*h),
            _ => {}
        }
    }
    found.sort();
    found.dedup();
    match found.as_slice() {
        [h] => Verdict::Ok(*h),
        [] => Verdict::Refused(refuse_seal(
            &sealed.0,
            &format!("seal {}: no cell declares this native", sealed.0),
        )),
        _ => Verdict::Refused(refuse_seal(
            &sealed.0,
            &format!(
                "seal {}: two cells declare this native ({}, {})",
                sealed.0,
                found[0].to_hex(),
                found[1].to_hex()
            ),
        )),
    }
}
