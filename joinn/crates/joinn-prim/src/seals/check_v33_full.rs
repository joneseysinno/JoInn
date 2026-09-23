//! Full V33: matter register, cell walk, seal dag.

use crate::floor::{self, Register};
use joinn_dna::{Body, Cell, NativeId};
use joinn_frame::{Hash, Verdict};
use std::collections::{BTreeMap, BTreeSet};

use super::Seal;

/// Full V33: matter register, cell walk, seal dag.
pub fn check_v33_full(
    body: &Body,
    sealed: &NativeId,
    floor: &BTreeSet<String>,
    self_cell: Hash,
    seals: &[Seal],
    cells: &BTreeMap<Hash, Cell>,
    bodies: &BTreeMap<Hash, Body>,
) -> Verdict<()> {
    let matter: BTreeSet<String> = floor::register()
        .iter()
        .filter(|p| p.register() == Register::Matter)
        .map(|p| p.name().to_string())
        .collect();
    let edges: Vec<joinn_gate::check::v33::SealEdge> = seals
        .iter()
        .map(|s| joinn_gate::check::v33::SealEdge {
            cell: s.cell,
            sealed: s.sealed.0.clone(),
            reference: s.reference.hash,
        })
        .collect();
    joinn_gate::check::v33::check(
        body, &sealed.0, floor, &matter, self_cell, &edges, cells, bodies,
    )
}
