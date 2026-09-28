//! Mark one instance's cell refused.

use joinn_frame::Verdict;

use super::Scene;
use crate::refuse::refuse;
use crate::tables::{CellRow, Delta, REFUSED, Row, RowWrite, STYLE_CELL_REFUSED};

impl Scene {
    /// Sets the cell's refused bit and refused style; nothing else moves.
    pub(crate) fn mark_refused(&mut self, instance: &str) -> Verdict<Delta> {
        let Some(&slot) = self.cells.get(instance) else {
            return refuse(format!(
                "regrow: refusal site {instance} is not in this scene; acceptance is an instance of body {}",
                self.alias
            ));
        };
        let Some(&old) = self.tables.cell.get(slot as usize) else {
            return Verdict::Ok(Delta::default());
        };
        let row = Row::Cell(CellRow {
            flags: old.flags | REFUSED,
            style: STYLE_CELL_REFUSED,
            ..old
        });
        let rows = match self.tables.put(slot, row) {
            Some(table) => vec![RowWrite { table, slot }],
            None => Vec::new(),
        };
        self.pending.extend(rows.iter().copied());
        Verdict::Ok(Delta { rows })
    }
}
