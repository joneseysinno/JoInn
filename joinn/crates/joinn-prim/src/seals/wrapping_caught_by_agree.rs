//! Mutant 13: wrapping add vs the reference body.

use joinn_dna::{Body, Cell};
use joinn_frame::{Hash, Verdict};
use std::collections::BTreeMap;

use super::agree_one::agree_one;
use super::{DnaFire, Seal};

/// Mutant 13: wrapping add vs the reference body. Must refuse as a truth violation.
#[cfg(any(test, feature = "mutants"))]
pub fn wrapping_caught_by_agree(
    dna: &dyn DnaFire,
    bodies: &BTreeMap<Hash, Body>,
    cells: &BTreeMap<Hash, Cell>,
    seal: &Seal,
    seed: u64,
    n: u32,
) -> bool {
    matches!(
        agree_one(
            dna,
            bodies,
            cells,
            seal,
            &crate::mutants::Wrapping,
            seed,
            n,
        ),
        Verdict::Refused(r)
            if r.reason.contains("TRUTH VIOLATION") && r.counterexample.is_some()
    )
}
