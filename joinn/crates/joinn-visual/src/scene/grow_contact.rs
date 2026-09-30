//! Grow a scene from a contact body: its cells touching, its responses latent.

use std::collections::{BTreeMap, BTreeSet};

use joinn_dna::{Cell, Contact};
use joinn_frame::{Hash, Verdict};

use super::Scene;
use crate::layout::layout_contact;
use crate::tables::{CellRow, LATENT, Row, Tables};

impl Scene {
    /// Cells in name order, then each cell's surface ports by position, and no
    /// link rows (R80). A response grows latent: nothing has reached it yet.
    pub fn grow_contact(
        alias: &str,
        contact: &Contact,
        cells: &BTreeMap<Hash, Cell>,
    ) -> Verdict<Scene> {
        let placed = match layout_contact(contact, cells) {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let responses: Vec<String> = placed
            .cells
            .iter()
            .filter(|c| c.response)
            .map(|c| c.instance.clone())
            .collect();
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
        for name in &responses {
            let Some(&slot) = scene.cells.get(name) else {
                continue;
            };
            if let Some(&old) = scene.tables.cell.get(slot as usize) {
                scene.tables.put(
                    slot,
                    Row::Cell(CellRow {
                        flags: old.flags | LATENT,
                        ..old
                    }),
                );
            }
        }
        scene.pending.clear();
        Verdict::Ok(scene)
    }
}
