//! Present one description: its ports' filled bits, and the refused bits.

use std::collections::BTreeSet;

use joinn_frame::Verdict;
use joinn_host::{Description, Role};
use joinn_link::Address;

use super::Scene;
use crate::refuse::refuse;
use crate::tables::{
    CellRow, Delta, FILLED, PortRow, REFUSED, Row, RowWrite, STYLE_CELL, STYLE_CELL_REFUSED,
};

impl Scene {
    /// A port is filled when its face holds a value. A refusal marks its own cell
    /// refused; any other description clears every refused bit, as `BodyState`
    /// clears `last_refusal` when anything fires. Only changed rows enter the delta.
    pub fn present(&mut self, d: &Description) -> Verdict<Delta> {
        let Some(&cell) = self.cells.get(&d.instance) else {
            return refuse(format!(
                "present: instance {} is not in this scene; acceptance is an instance of body {}",
                d.instance, self.alias
            ));
        };
        let mut writes: Vec<(u32, Row)> = Vec::new();
        for face in &d.ports {
            let address = Address {
                instance: d.instance.clone(),
                port: face.position,
            };
            let Some(&slot) = self.ports.get(&address) else {
                return refuse(format!(
                    "present: port {} is not in this scene; acceptance is a port of body {}",
                    address.printed(),
                    self.alias
                ));
            };
            if let Some(&old) = self.tables.port.get(slot as usize) {
                let flags = if face.value.is_some() {
                    old.flags | FILLED
                } else {
                    old.flags & !FILLED
                };
                writes.push((slot, Row::Port(PortRow { flags, ..old })));
            }
        }
        let refused = d.role == Role::Refusal;
        for &slot in self.cells.values() {
            let Some(&old) = self.tables.cell.get(slot as usize) else {
                continue;
            };
            if refused && slot == cell {
                writes.push((
                    slot,
                    Row::Cell(CellRow {
                        flags: old.flags | REFUSED,
                        style: STYLE_CELL_REFUSED,
                        ..old
                    }),
                ));
            } else if !refused {
                writes.push((
                    slot,
                    Row::Cell(CellRow {
                        flags: old.flags & !REFUSED,
                        style: STYLE_CELL,
                        ..old
                    }),
                ));
            }
        }
        let mut changed: BTreeSet<RowWrite> = BTreeSet::new();
        for (slot, row) in writes {
            if let Some(table) = self.tables.put(slot, row) {
                changed.insert(RowWrite { table, slot });
            }
        }
        self.pending.extend(changed.iter().copied());
        Verdict::Ok(Delta {
            rows: changed.into_iter().collect(),
        })
    }
}
