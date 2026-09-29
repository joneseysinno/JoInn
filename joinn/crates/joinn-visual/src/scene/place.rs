//! Reconcile the tables with a layout: free what is gone, then place everything
//! in growth order. A kept row keeps its slot, generation and flags.

use std::collections::BTreeSet;

use joinn_dna::Direction;
use joinn_frame::Verdict;
use joinn_link::Address;

use super::Scene;
use crate::layout::{CELL_RADIUS, Layout, PORT_RADIUS, SURFACE_RADIUS, WIRE_HALF_WIDTH_QUARTERS};
use crate::refuse::refuse;
use crate::tables::{
    BodyRow, CellRow, Delta, LIVE, LinkRow, PortRow, Row, RowWrite, STYLE_CELL, Table, WIRE_KIND,
};

impl Scene {
    /// Frees first: a freed row has `live` cleared and its generation incremented.
    /// Then cells by name, each cell's ports by position, and wires in print
    /// order. A new row takes the lowest free slot and that slot's generation, or
    /// generation 1 on a slot never used. Freeing a link leaves its incidence
    /// entries alone; they are rewritten when the slot is reused.
    pub(crate) fn place(&mut self, next: Layout) -> Verdict<Delta> {
        macro_rules! fit {
            ($v:expr) => {
                match ($v).try_into() {
                    Ok(n) => n,
                    Err(_) => {
                        return refuse(format!(
                            "scene: {} does not fit a table field; acceptance is a layout within 32 bits",
                            $v
                        ));
                    }
                }
            };
        }
        let mut changed: BTreeSet<RowWrite> = BTreeSet::new();
        macro_rules! put_row {
            ($slot:expr, $row:expr) => {{
                let slot: u32 = $slot;
                if let Some(table) = self.tables.put(slot, $row) {
                    changed.insert(RowWrite { table, slot });
                }
            }};
        }
        macro_rules! bump {
            ($table:expr, $name:expr, $slot:expr) => {
                match $table.generation.checked_add(1) {
                    Some(g) => g,
                    None => {
                        return refuse(format!(
                            "scene: {} slot {} generation overflowed; acceptance is fewer than 2^32 frees of one slot",
                            $name, $slot
                        ));
                    }
                }
            };
        }

        let keep_cells: BTreeSet<&str> = next.cells.iter().map(|c| c.instance.as_str()).collect();
        let mut keep_ports: BTreeSet<&Address> = BTreeSet::new();
        for p in &next.ports {
            if !keep_ports.insert(&p.address) {
                return refuse(format!(
                    "scene: {} names an in-port and an out-port; acceptance is a body whose cells give each position one port, since an ID names a port by position",
                    p.address.printed()
                ));
            }
        }
        let keep_links: BTreeSet<(&Address, &Address)> =
            next.wires.iter().map(|w| (&w.src, &w.dst)).collect();
        let gone_cells: Vec<(String, u32)> = self
            .cells
            .iter()
            .filter(|(n, _)| !keep_cells.contains(n.as_str()))
            .map(|(n, s)| (n.clone(), *s))
            .collect();
        let gone_ports: Vec<(Address, u32)> = self
            .ports
            .iter()
            .filter(|(a, _)| !keep_ports.contains(a))
            .map(|(a, s)| (a.clone(), *s))
            .collect();
        let gone_links: Vec<((Address, Address), u32)> = self
            .links
            .iter()
            .filter(|((s, d), _)| !keep_links.contains(&(s, d)))
            .map(|(k, s)| (k.clone(), *s))
            .collect();
        for (name, slot) in gone_cells {
            if let Some(&old) = self.tables.cell.get(slot as usize) {
                let generation = bump!(old, "cell", slot);
                let flags = old.flags & !LIVE;
                put_row!(
                    slot,
                    Row::Cell(CellRow {
                        generation,
                        flags,
                        ..old
                    })
                );
            }
            self.cells.remove(&name);
        }
        for (address, slot) in gone_ports {
            if let Some(&old) = self.tables.port.get(slot as usize) {
                let generation = bump!(old, "port", slot);
                let flags = old.flags & !LIVE;
                put_row!(
                    slot,
                    Row::Port(PortRow {
                        generation,
                        flags,
                        ..old
                    })
                );
            }
            self.ports.remove(&address);
        }
        for (key, slot) in gone_links {
            if let Some(&old) = self.tables.link.get(slot as usize) {
                let generation = bump!(old, "link", slot);
                let flags = old.flags & !LIVE;
                put_row!(
                    slot,
                    Row::Link(LinkRow {
                        generation,
                        flags,
                        ..old
                    })
                );
            }
            self.links.remove(&key);
        }

        let m = next.surface;
        let body = BodyRow {
            x: fit!(m.x),
            y: fit!(m.y),
            w: fit!(m.w),
            h: fit!(m.h),
            radius: fit!(SURFACE_RADIUS),
            generation: self.tables.body.first().map_or(1, |r| r.generation),
            flags: LIVE,
        };
        put_row!(0, Row::Body(body));

        for c in &next.cells {
            let kept = self.cells.get(&c.instance).copied();
            let slot = kept.unwrap_or_else(|| self.tables.free_slot(Table::Cell));
            let old = self.tables.cell.get(slot as usize).copied();
            let (style, flags) = match (kept, old) {
                (Some(_), Some(o)) => (o.style, o.flags),
                _ => (STYLE_CELL, LIVE),
            };
            let row = CellRow {
                body: 0,
                x: fit!(c.rect.x),
                y: fit!(c.rect.y),
                w: fit!(c.rect.w),
                h: fit!(c.rect.h),
                radius: fit!(CELL_RADIUS),
                generation: old.map_or(1, |o| o.generation),
                style,
                flags,
            };
            put_row!(slot, Row::Cell(row));
            self.cells.insert(c.instance.clone(), slot);
        }

        for p in &next.ports {
            let Some(&cell) = self.cells.get(&p.address.instance) else {
                return refuse(format!(
                    "scene: port {} has no cell; acceptance is a layout whose ports sit on its cells",
                    p.address.printed()
                ));
            };
            let kept = self.ports.get(&p.address).copied();
            let slot = kept.unwrap_or_else(|| self.tables.free_slot(Table::Port));
            let old = self.tables.port.get(slot as usize).copied();
            let flags = match (kept, old) {
                (Some(_), Some(o)) => o.flags,
                _ => LIVE,
            };
            let row = PortRow {
                cell,
                position: p.address.port,
                direction: match p.direction {
                    Direction::In => 0,
                    Direction::Out => 1,
                },
                x: fit!(p.x),
                y: fit!(p.y),
                radius: fit!(PORT_RADIUS),
                generation: old.map_or(1, |o| o.generation),
                flags,
            };
            put_row!(slot, Row::Port(row));
            self.ports.insert(p.address.clone(), slot);
        }

        for w in &next.wires {
            let (Some(&src), Some(&dst)) = (self.ports.get(&w.src), self.ports.get(&w.dst)) else {
                return refuse(format!(
                    "scene: wire {} -> {} names a port the layout lacks; acceptance is a layout whose wires join its ports",
                    w.src.printed(),
                    w.dst.printed()
                ));
            };
            let key = (w.src.clone(), w.dst.clone());
            let slot = match self.links.get(&key) {
                Some(&s) => s,
                None => self.tables.free_slot(Table::Link),
            };
            let old = self.tables.link.get(slot as usize).copied();
            let Some(start) = slot.checked_mul(2) else {
                return refuse(format!(
                    "scene: link slot {slot} has no incidence entry; acceptance is fewer than 2^31 links"
                ));
            };
            let row = LinkRow {
                kind: WIRE_KIND,
                start,
                count: 2,
                body: 0,
                half_width_quarters: fit!(WIRE_HALF_WIDTH_QUARTERS),
                generation: old.map_or(1, |o| o.generation),
                flags: LIVE,
            };
            put_row!(slot, Row::Link(row));
            put_row!(start, Row::Incidence(src));
            put_row!(start | 1, Row::Incidence(dst));
            self.links.insert(key, slot);
        }

        self.layout = next;
        self.pending.extend(changed.iter().copied());
        Verdict::Ok(Delta {
            rows: changed.into_iter().collect(),
        })
    }
}
