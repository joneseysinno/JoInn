//! Write a system's rows from its layout and its count.

use joinn_dna::Direction;
use joinn_frame::{Value, Verdict};

use super::SystemScene;
use crate::font::port_value;
use crate::layout::{CELL_RADIUS, PORT_RADIUS, SURFACE_RADIUS};
use crate::pick::{FORCE_TAG, PORT_TAG};
use crate::refuse::refuse;
use crate::system_layout::{LASSO_HALF_WIDTH, SYSTEM_PAD, SystemLayout};
use crate::tables::{
    BodyRow, CHART_BODY, CellRow, ChartRow, Delta, FILLED, LATENT, LIVE, PortRow, Row, RowWrite,
    STYLE_CELL, STYLE_FORCE, STYLE_TEXT, StrokeRow, Table,
};

impl SystemScene {
    /// Body 0 and chart 0 (the body's, at origin 0). Cell and port slot 0 are
    /// the response and its out-port; slot `k + 1` is `numbers.k` and its
    /// in-port, filled, or a waiting box (latent, its in-port empty). Stroke
    /// slots 0 … 10 are the lasso, owned by the force, then the count's
    /// value strokes. Every row is written and only rows whose bytes changed
    /// enter the delta; rows past a table's new end are dropped and reported,
    /// so the tables equal a regrow's. Nothing is freed: a body only grows.
    pub(super) fn write_rows(&mut self, next: SystemLayout, count: &Value) -> Verdict<Delta> {
        macro_rules! fit {
            ($v:expr) => {
                match ($v).try_into() {
                    Ok(n) => n,
                    Err(_) => {
                        return refuse(format!(
                            "scene: {} does not fit a table field; acceptance is a system within 32 bits",
                            $v
                        ));
                    }
                }
            };
        }
        let filled = next.cells.len();
        let boxes: Vec<_> = next.cells.iter().chain(&next.waiting).collect();
        let mut rows: Vec<(u32, Row)> = Vec::new();
        let s = next.surface;
        rows.push((
            0,
            Row::Body(BodyRow {
                x: fit!(s.x),
                y: fit!(s.y),
                w: fit!(s.w),
                h: fit!(s.h),
                radius: fit!(SURFACE_RADIUS),
                generation: 1,
                flags: LIVE,
            }),
        ));
        let r = next.response.rect;
        let extent = (r.x + r.w).max(r.y + r.h).max(s.y + s.h) + SYSTEM_PAD;
        rows.push((
            0,
            Row::Chart(ChartRow {
                origin_x: 0,
                origin_y: 0,
                parent: 0,
                kind: CHART_BODY,
                size: fit!(extent),
                generation: 1,
                flags: LIVE,
            }),
        ));
        let cell_row = |rect: crate::layout::Rect, flags: u32| -> Option<CellRow> {
            Some(CellRow {
                body: 0,
                x: rect.x.try_into().ok()?,
                y: rect.y.try_into().ok()?,
                w: rect.w.try_into().ok()?,
                h: rect.h.try_into().ok()?,
                radius: CELL_RADIUS.try_into().ok()?,
                generation: 1,
                style: STYLE_CELL,
                flags,
            })
        };
        let mut cells = vec![(r, LIVE)];
        cells.extend(
            boxes
                .iter()
                .enumerate()
                .map(|(k, b)| (b.rect, if k < filled { LIVE } else { LIVE | LATENT })),
        );
        for (slot, (rect, flags)) in (0u32..).zip(cells) {
            let Some(row) = cell_row(rect, flags) else {
                return refuse(format!(
                    "scene: cell slot {slot} does not fit a table field; acceptance is a system within 32 bits"
                ));
            };
            rows.push((slot, Row::Cell(row)));
        }
        let mut out_port = None;
        for p in &next.ports {
            let (slot, flags) = if p.direction == Direction::Out {
                out_port = Some(p);
                (0u32, LIVE | FILLED)
            } else {
                let Some(k) = boxes.iter().position(|b| b.instance == p.address.instance) else {
                    return refuse(format!(
                        "scene: port {} has no box; acceptance is a layout whose ports sit on its boxes",
                        p.address.printed()
                    ));
                };
                let flags = if k < filled { LIVE | FILLED } else { LIVE };
                (fit!(k + 1), flags)
            };
            rows.push((
                slot,
                Row::Port(PortRow {
                    cell: slot,
                    position: p.address.port,
                    direction: u32::from(p.direction == Direction::Out),
                    x: fit!(p.x),
                    y: fit!(p.y),
                    radius: fit!(PORT_RADIUS),
                    generation: 1,
                    flags,
                }),
            ));
        }
        let Some(out_port) = out_port else {
            return refuse("scene: the response has no out-port; acceptance is a layout with one");
        };
        let mut strokes: Vec<StrokeRow> = Vec::new();
        for l in &next.lasso {
            strokes.push(StrokeRow {
                chart: 0,
                x0: fit!(16 * l.from.0),
                y0: fit!(16 * l.from.1),
                x1: fit!(16 * l.to.0),
                y1: fit!(16 * l.to.1),
                half_width: fit!(LASSO_HALF_WIDTH),
                owner: [1, 0, FORCE_TAG],
                style: STYLE_FORCE,
                generation: 1,
                flags: LIVE,
            });
        }
        let text = match port_value(out_port, &count.print_term()) {
            Verdict::Ok(t) => t,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        for t in text {
            strokes.push(StrokeRow {
                chart: 0,
                x0: fit!(t.x0),
                y0: fit!(t.y0),
                x1: fit!(t.x1),
                y1: fit!(t.y1),
                half_width: fit!(t.half_width),
                owner: [1, 1, PORT_TAG | out_port.address.port],
                style: STYLE_TEXT,
                generation: 1,
                flags: LIVE,
            });
        }
        let lens = [
            (Table::Cell, boxes.len() + 1),
            (Table::Port, next.ports.len()),
            (Table::Stroke, strokes.len()),
        ];
        rows.extend(
            (0u32..)
                .zip(strokes)
                .map(|(slot, s)| (slot, Row::Stroke(s))),
        );
        let mut changed: Vec<RowWrite> = Vec::new();
        for (slot, row) in rows {
            if let Some(table) = self.tables.put(slot, row) {
                changed.push(RowWrite { table, slot });
            }
        }
        for (table, len) in lens {
            let have = match table {
                Table::Cell => self.tables.cell.len(),
                Table::Port => self.tables.port.len(),
                _ => self.tables.stroke.len(),
            };
            for slot in len..have {
                changed.push(RowWrite {
                    table,
                    slot: fit!(slot),
                });
            }
            match table {
                Table::Cell => self.tables.cell.truncate(len),
                Table::Port => self.tables.port.truncate(len),
                _ => self.tables.stroke.truncate(len),
            }
        }
        changed.sort();
        changed.dedup();
        self.layout = next;
        self.pending.extend(changed.iter().copied());
        Verdict::Ok(Delta { rows: changed })
    }
}
