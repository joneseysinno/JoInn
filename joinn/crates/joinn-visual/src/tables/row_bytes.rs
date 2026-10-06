//! One row's bytes: its fields little-endian in field order, padded with zeros.

use super::{
    BODY_ROW_BYTES, CELL_ROW_BYTES, CHART_ROW_BYTES, FRAME_ROW_BYTES, INCIDENCE_BYTES,
    LINK_ROW_BYTES, PORT_ROW_BYTES, ROUTE_ROW_BYTES, SEGMENT_ROW_BYTES, STROKE_ROW_BYTES, Table,
    Tables,
};

/// `None` when the slot is past the end of the table.
pub fn row_bytes(tables: &Tables, table: Table, slot: u32) -> Option<Vec<u8>> {
    let at = slot as usize;
    let (words, size): (Vec<[u8; 4]>, usize) = match table {
        Table::Body => {
            let r = tables.body.get(at)?;
            (
                vec![
                    r.x.to_le_bytes(),
                    r.y.to_le_bytes(),
                    r.w.to_le_bytes(),
                    r.h.to_le_bytes(),
                    r.radius.to_le_bytes(),
                    r.generation.to_le_bytes(),
                    r.flags.to_le_bytes(),
                ],
                BODY_ROW_BYTES,
            )
        }
        Table::Cell => {
            let r = tables.cell.get(at)?;
            (
                vec![
                    r.body.to_le_bytes(),
                    r.x.to_le_bytes(),
                    r.y.to_le_bytes(),
                    r.w.to_le_bytes(),
                    r.h.to_le_bytes(),
                    r.radius.to_le_bytes(),
                    r.generation.to_le_bytes(),
                    r.style.to_le_bytes(),
                    r.flags.to_le_bytes(),
                ],
                CELL_ROW_BYTES,
            )
        }
        Table::Port => {
            let r = tables.port.get(at)?;
            (
                vec![
                    r.cell.to_le_bytes(),
                    r.position.to_le_bytes(),
                    r.direction.to_le_bytes(),
                    r.x.to_le_bytes(),
                    r.y.to_le_bytes(),
                    r.radius.to_le_bytes(),
                    r.generation.to_le_bytes(),
                    r.flags.to_le_bytes(),
                ],
                PORT_ROW_BYTES,
            )
        }
        Table::Link => {
            let r = tables.link.get(at)?;
            (
                vec![
                    r.kind.to_le_bytes(),
                    r.start.to_le_bytes(),
                    r.count.to_le_bytes(),
                    r.body.to_le_bytes(),
                    r.half_width_quarters.to_le_bytes(),
                    r.generation.to_le_bytes(),
                    r.flags.to_le_bytes(),
                ],
                LINK_ROW_BYTES,
            )
        }
        Table::Incidence => (
            vec![tables.incidence.get(at)?.to_le_bytes()],
            INCIDENCE_BYTES,
        ),
        Table::Chart => {
            let r = tables.chart.get(at)?;
            (
                vec![
                    r.origin_x.to_le_bytes(),
                    r.origin_y.to_le_bytes(),
                    r.parent.to_le_bytes(),
                    r.kind.to_le_bytes(),
                    r.size.to_le_bytes(),
                    r.generation.to_le_bytes(),
                    r.flags.to_le_bytes(),
                ],
                CHART_ROW_BYTES,
            )
        }
        Table::Frame => {
            let r = tables.frame.get(at)?;
            (
                vec![
                    r.chart.to_le_bytes(),
                    r.w.to_le_bytes(),
                    r.h.to_le_bytes(),
                    r.kind.to_le_bytes(),
                    r.index.to_le_bytes(),
                    r.generation.to_le_bytes(),
                    r.flags.to_le_bytes(),
                ],
                FRAME_ROW_BYTES,
            )
        }
        Table::Stroke => {
            let r = tables.stroke.get(at)?;
            (
                vec![
                    r.chart.to_le_bytes(),
                    r.x0.to_le_bytes(),
                    r.y0.to_le_bytes(),
                    r.x1.to_le_bytes(),
                    r.y1.to_le_bytes(),
                    r.half_width.to_le_bytes(),
                    r.owner[0].to_le_bytes(),
                    r.owner[1].to_le_bytes(),
                    r.owner[2].to_le_bytes(),
                    r.style.to_le_bytes(),
                    r.generation.to_le_bytes(),
                    r.flags.to_le_bytes(),
                ],
                STROKE_ROW_BYTES,
            )
        }
        Table::Route => {
            let r = tables.route.get(at)?;
            (
                vec![
                    r.link.to_le_bytes(),
                    r.fold.to_le_bytes(),
                    r.size.to_le_bytes(),
                    r.ordered.to_le_bytes(),
                    r.segment_first.to_le_bytes(),
                    r.segment_count.to_le_bytes(),
                    r.generation.to_le_bytes(),
                    r.flags.to_le_bytes(),
                ],
                ROUTE_ROW_BYTES,
            )
        }
        Table::Segment => {
            let r = tables.segment.get(at)?;
            (
                vec![
                    r.chart.to_le_bytes(),
                    r.x0.to_le_bytes(),
                    r.y0.to_le_bytes(),
                    r.x1.to_le_bytes(),
                    r.y1.to_le_bytes(),
                    r.route.to_le_bytes(),
                    r.member.to_le_bytes(),
                    r.kind.to_le_bytes(),
                    r.legs.to_le_bytes(),
                    r.style.to_le_bytes(),
                    r.generation.to_le_bytes(),
                    r.flags.to_le_bytes(),
                ],
                SEGMENT_ROW_BYTES,
            )
        }
    };
    let mut out: Vec<u8> = words.into_iter().flatten().collect();
    out.resize(size, 0);
    Some(out)
}
