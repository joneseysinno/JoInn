//! Rebuild a system scene from the system and the whole input list.

use std::collections::{BTreeMap, BTreeSet};

use joinn_dna::{Cell, Contact, System};
use joinn_frame::{FrameRegistry, Hash, Value, Verdict};
use joinn_link::{grow, respond};

use super::SystemScene;
use crate::system_layout::layout_system;
use crate::tables::Tables;

impl SystemScene {
    /// Grow `inputs` from nothing, ask the engine for the count, lay out, and
    /// write every row. Nothing is left pending. The test of every growth
    /// delta (rule 58).
    pub fn regrow(
        system: &System,
        contacts: &BTreeMap<Hash, Contact>,
        cells: &BTreeMap<Hash, Cell>,
        waiting: u32,
        inputs: &[Value],
    ) -> Verdict<SystemScene> {
        let grown = match grow(system, contacts, cells, inputs) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let count = match respond(&grown, cells, &FrameRegistry::phase1()) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let layout = match layout_system(system, contacts, cells, &grown, waiting) {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let mut scene = SystemScene {
            contacts: contacts.clone(),
            cells: cells.clone(),
            waiting,
            grown,
            count: count.clone(),
            layout: layout.clone(),
            tables: Tables::empty(),
            pending: BTreeSet::new(),
        };
        if let Verdict::Refused(r) = scene.write_rows(layout, &count) {
            return Verdict::Refused(r);
        }
        scene.pending.clear();
        Verdict::Ok(scene)
    }
}
