//! Every table's bytes, in `ROW_SIZES` order.

use joinn_visual::{Tables, table_bytes};

/// Body, cell, port, link, incidence, style, chart, frame, stroke.
pub(super) fn all_bytes(tables: &Tables) -> [Vec<u8>; 9] {
    let b = table_bytes(tables);
    [
        b.body,
        b.cell,
        b.port,
        b.link,
        b.incidence,
        b.style,
        b.chart,
        b.frame,
        b.stroke,
    ]
}
