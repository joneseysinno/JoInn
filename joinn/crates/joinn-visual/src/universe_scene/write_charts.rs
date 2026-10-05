//! Write every chart row relative to the scene's anchor.

use joinn_frame::Verdict;

use super::UniverseScene;
use super::fit::fit;
use crate::charts::ChartKind;
use crate::tables::{
    CHART_BODY, CHART_GALAXY, CHART_SYSTEM, CHART_UNIVERSE, ChartRow, LIVE, Row, RowWrite,
};

impl UniverseScene {
    /// Each chart's origin is its root origin less the anchor's; its parent is
    /// its parent's slot. Returns the rows whose bytes changed, at most one per
    /// chart.
    pub(super) fn write_charts(&mut self) -> Verdict<Vec<RowWrite>> {
        let anchor = self
            .layout
            .chart(self.anchor)
            .map_or((0, 0), |c| c.root_origin);
        let mut rows = Vec::new();
        for (i, c) in self.layout.charts.iter().enumerate() {
            let (Some(&slot), Some(&parent)) =
                (self.slots.get(i), self.slots.get(c.parent.0 as usize))
            else {
                continue;
            };
            let (origin_x, origin_y, size) = match (
                fit(c.root_origin.0 - anchor.0),
                fit(c.root_origin.1 - anchor.1),
                fit(c.size.0.max(c.size.1)),
            ) {
                (Verdict::Ok(x), Verdict::Ok(y), Verdict::Ok(s)) => (x, y, s),
                (Verdict::Refused(r), _, _)
                | (_, Verdict::Refused(r), _)
                | (_, _, Verdict::Refused(r)) => return Verdict::Refused(r),
            };
            let kind = match c.kind {
                ChartKind::Universe => CHART_UNIVERSE,
                ChartKind::Galaxy => CHART_GALAXY,
                ChartKind::System => CHART_SYSTEM,
                ChartKind::Body => CHART_BODY,
            };
            let row = ChartRow {
                origin_x,
                origin_y,
                parent,
                kind,
                size,
                generation: 1,
                flags: LIVE,
            };
            if let Some(table) = self.tables.put(slot, Row::Chart(row)) {
                rows.push(RowWrite { table, slot });
            }
        }
        self.pending.extend(rows.iter().copied());
        Verdict::Ok(rows)
    }
}
