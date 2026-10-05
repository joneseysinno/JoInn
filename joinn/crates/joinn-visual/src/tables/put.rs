//! Write one row. Reports whether its bytes changed.

use super::{Row, Table, Tables};

impl Tables {
    /// A slot past the end grows the table with rows that do not draw. Returns
    /// the row's table when its bytes changed, `None` when they didn't.
    pub(crate) fn put(&mut self, slot: u32, row: Row) -> Option<Table> {
        let at = slot as usize;
        macro_rules! set {
            ($rows:expr, $row:expr) => {{
                let rows = &mut $rows;
                let grew = rows.len() <= at;
                if grew {
                    rows.resize(at + 1, Default::default());
                }
                let changed = grew || rows[at] != $row;
                rows[at] = $row;
                changed
            }};
        }
        let (table, changed) = match row {
            Row::Body(r) => (Table::Body, set!(self.body, r)),
            Row::Cell(r) => (Table::Cell, set!(self.cell, r)),
            Row::Port(r) => (Table::Port, set!(self.port, r)),
            Row::Link(r) => (Table::Link, set!(self.link, r)),
            Row::Incidence(r) => (Table::Incidence, set!(self.incidence, r)),
            Row::Chart(r) => (Table::Chart, set!(self.chart, r)),
            Row::Frame(r) => (Table::Frame, set!(self.frame, r)),
            Row::Stroke(r) => (Table::Stroke, set!(self.stroke, r)),
        };
        changed.then_some(table)
    }
}
