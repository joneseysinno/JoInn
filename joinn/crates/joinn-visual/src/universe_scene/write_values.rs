//! Rewrite the value strokes from every kept value.

use std::collections::BTreeMap;

use joinn_frame::Verdict;
use joinn_link::Address;

use super::UniverseScene;
use super::fit::fit;
use crate::font::port_value;
use crate::tables::{LIVE, Row, RowWrite, STYLE_TEXT, StrokeRow, Table};

impl UniverseScene {
    /// The value strokes are the last stroke rows, from `value_start`, one run
    /// per kept value in (body slot, address) order. Nothing is written when a
    /// value has a character outside the stroke set. Rows past the new end are
    /// dropped and reported, so the table equals a regrow's.
    pub(super) fn write_values(
        &mut self,
        values: BTreeMap<(u32, Address), String>,
    ) -> Verdict<Vec<RowWrite>> {
        let mut rows: Vec<StrokeRow> = Vec::new();
        for ((body, address), value) in &values {
            let Some(&slot) = self.ports.get(&(*body, address.clone())) else {
                continue;
            };
            let Some(chart) = self
                .layout
                .charts
                .iter()
                .find(|c| c.body.is_some() && c.index == *body)
            else {
                continue;
            };
            let Some(dot) = chart
                .body
                .and_then(|(_, li)| self.layout.layouts.get(li))
                .and_then(|l| l.ports.iter().find(|p| &p.address == address))
            else {
                continue;
            };
            let Some(port) = self.tables.port.get(slot as usize).copied() else {
                continue;
            };
            let strokes = match port_value(dot, value) {
                Verdict::Ok(s) => s,
                Verdict::Refused(r) => return Verdict::Refused(r),
            };
            for s in strokes {
                let field = |v: i64| fit::<i32>(v);
                let (x0, y0, x1, y1, hw) = match (
                    field(s.x0),
                    field(s.y0),
                    field(s.x1),
                    field(s.y1),
                    fit::<u32>(s.half_width),
                ) {
                    (
                        Verdict::Ok(a),
                        Verdict::Ok(b),
                        Verdict::Ok(c),
                        Verdict::Ok(d),
                        Verdict::Ok(e),
                    ) => (a, b, c, d, e),
                    _ => {
                        return crate::refuse::refuse(format!(
                            "present: the value of {} does not fit a stroke row; acceptance is a value within 32 bits",
                            address.printed()
                        ));
                    }
                };
                rows.push(StrokeRow {
                    chart: *body,
                    x0,
                    y0,
                    x1,
                    y1,
                    half_width: hw,
                    owner: [
                        body + 1,
                        port.cell + 1,
                        crate::pick::PORT_TAG | address.port,
                    ],
                    style: STYLE_TEXT,
                    generation: port.generation,
                    flags: LIVE,
                });
            }
        }
        let mut changed = Vec::new();
        let start = self.value_start;
        for (slot, row) in (start..).zip(&rows) {
            if let Some(table) = self.tables.put(slot, Row::Stroke(*row)) {
                changed.push(RowWrite { table, slot });
            }
        }
        let end = start as usize + rows.len();
        if self.tables.stroke.len() > end {
            for slot in end..self.tables.stroke.len() {
                if let Ok(slot) = u32::try_from(slot) {
                    changed.push(RowWrite {
                        table: Table::Stroke,
                        slot,
                    });
                }
            }
            self.tables.stroke.truncate(end);
        }
        self.values = values;
        Verdict::Ok(changed)
    }
}
