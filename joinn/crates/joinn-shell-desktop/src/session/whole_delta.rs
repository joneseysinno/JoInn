//! Every row of a picture, for a renderer that holds another picture's rows.

use joinn_visual::{Delta, RowWrite, Table, Tables};

/// Each slot of the body, chart and stroke tables. A table whose length
/// differs from what the renderer holds is written whole anyway; one of the
/// same length is written slot by slot.
pub(super) fn whole_delta(tables: &Tables) -> Delta {
    let slots =
        |table: Table, len: usize| (0u32..).take(len).map(move |slot| RowWrite { table, slot });
    let rows = slots(Table::Body, tables.body.len())
        .chain(slots(Table::Chart, tables.chart.len()))
        .chain(slots(Table::Stroke, tables.stroke.len()))
        .collect();
    Delta { rows }
}
