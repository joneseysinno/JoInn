//! Grow a scene from a contact body: its cells touching, its responses latent.

use std::collections::{BTreeMap, BTreeSet};

use joinn_dna::{Cell, Contact};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_link::{Address, contact_surface, instance_ports, lower};

use super::Scene;
use crate::layout::layout_contact;
use crate::tables::{CellRow, LATENT, Row, Tables};

impl Scene {
    /// Cells in name order, then each cell's surface ports by position, and no
    /// link rows (R80). A response grows latent: nothing has reached it yet.
    /// Every port of the lowered body that is not on the contact's surface (a
    /// member port, a response in-port) is interior: `present` skips it.
    pub fn grow_contact(
        alias: &str,
        contact: &Contact,
        cells: &BTreeMap<Hash, Cell>,
    ) -> Verdict<Scene> {
        let placed = match layout_contact(contact, cells) {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let surface: BTreeSet<Address> = match contact_surface(contact, cells) {
            Verdict::Ok(s) => s.into_iter().map(|p| p.address).collect(),
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let lowered = match lower(contact, cells, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let interior: BTreeSet<Address> = match instance_ports(&lowered, cells) {
            Verdict::Ok(ports) => ports
                .into_iter()
                .flat_map(|(instance, decls)| {
                    decls.into_iter().map(move |p| Address {
                        instance: instance.clone(),
                        port: p.position,
                    })
                })
                .filter(|a| !surface.contains(a))
                .collect(),
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
            interior,
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
