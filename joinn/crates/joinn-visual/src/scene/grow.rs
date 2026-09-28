//! Grow a scene from DNA: every row placed in canonical order, nothing pending.

use std::collections::{BTreeMap, BTreeSet};

use joinn_dna::{Body, Cell};
use joinn_frame::{Hash, Verdict};

use super::Scene;
use crate::layout::layout;
use crate::tables::Tables;

impl Scene {
    /// Cells in name order, then each cell's ports by position, then wires in
    /// `print_body` order. Two grows of one body give the same bytes.
    pub fn grow(alias: &str, body: &Body, cells: &BTreeMap<Hash, Cell>) -> Verdict<Scene> {
        let placed = match layout(body, cells) {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let mut scene = Scene {
            alias: alias.to_owned(),
            layout: placed.clone(),
            tables: Tables::empty(),
            cells: BTreeMap::new(),
            ports: BTreeMap::new(),
            links: BTreeMap::new(),
            pending: BTreeSet::new(),
        };
        if let Verdict::Refused(r) = scene.place(placed) {
            return Verdict::Refused(r);
        }
        scene.pending.clear();
        Verdict::Ok(scene)
    }
}
