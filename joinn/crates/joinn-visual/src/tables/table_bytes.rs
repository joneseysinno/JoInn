//! Every table's bytes: what the GPU uploads and what regrow compares.

use super::row_bytes::row_bytes;
use super::{Table, TableBytes, Tables};

/// Rows in slot order, each encoded by `row_bytes`.
pub fn table_bytes(tables: &Tables) -> TableBytes {
    let all = |table: Table, rows: usize| -> Vec<u8> {
        (0..rows)
            .filter_map(|slot| row_bytes(tables, table, u32::try_from(slot).ok()?))
            .flatten()
            .collect()
    };
    TableBytes {
        body: all(Table::Body, tables.body.len()),
        cell: all(Table::Cell, tables.cell.len()),
        port: all(Table::Port, tables.port.len()),
        link: all(Table::Link, tables.link.len()),
        incidence: all(Table::Incidence, tables.incidence.len()),
        style: tables.style.iter().flat_map(|s| s.to_le_bytes()).collect(),
        chart: all(Table::Chart, tables.chart.len()),
        frame: all(Table::Frame, tables.frame.len()),
        stroke: all(Table::Stroke, tables.stroke.len()),
        route: all(Table::Route, tables.route.len()),
        segment: all(Table::Segment, tables.segment.len()),
    }
}

#[cfg(test)]
mod tests {
    use super::table_bytes;
    use crate::tables::{
        BODY_ROW_BYTES, BodyRow, CELL_ROW_BYTES, CHART_ROW_BYTES, CellRow, ChartRow,
        FRAME_ROW_BYTES, FrameRow, LINK_ROW_BYTES, PORT_ROW_BYTES, ROUTE_ROW_BYTES, RouteRow,
        SEGMENT_ROW_BYTES, STROKE_ROW_BYTES, SegmentRow, StrokeRow, Tables,
    };

    #[test]
    fn rows_are_little_endian_in_field_order_and_padded_to_sixteen() {
        let mut t = Tables::empty();
        t.body.push(BodyRow {
            x: -1,
            y: 2,
            w: 40,
            h: 24,
            radius: 3,
            generation: 1,
            flags: 1,
        });
        t.cell.push(CellRow {
            body: 0,
            x: 4,
            y: 4,
            w: 12,
            h: 6,
            radius: 2,
            generation: 1,
            style: 2,
            flags: 3,
        });
        let b = table_bytes(&t);
        assert_eq!(b.body.len(), BODY_ROW_BYTES);
        assert_eq!(&b.body[..8], &[0xFF, 0xFF, 0xFF, 0xFF, 2, 0, 0, 0]);
        assert_eq!(&b.body[24..], &[1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(b.cell.len(), CELL_ROW_BYTES);
        assert_eq!(&b.cell[28..36], &[2, 0, 0, 0, 3, 0, 0, 0]);
        assert_eq!(&b.cell[36..], &[0; 12]);
        assert_eq!(
            b.style[..4],
            [0x15, 0x17, 0x1C, 0xFF],
            "style 0 is the background #15171C"
        );
        for size in [
            BODY_ROW_BYTES,
            CELL_ROW_BYTES,
            PORT_ROW_BYTES,
            LINK_ROW_BYTES,
        ] {
            assert_eq!(size % 16, 0);
        }
    }

    #[test]
    fn chart_frame_and_stroke_rows_are_little_endian_in_field_order() {
        let mut t = Tables::empty();
        t.chart.push(ChartRow {
            origin_x: -88,
            origin_y: 112,
            parent: 3073,
            kind: 3,
            size: 40,
            generation: 1,
            flags: 1,
        });
        t.frame.push(FrameRow {
            chart: 3072,
            w: 304,
            h: 152,
            kind: 2,
            index: 5,
            generation: 1,
            flags: 1,
        });
        t.stroke.push(StrokeRow {
            chart: 0,
            x0: 400,
            y0: 76,
            x1: 412,
            y1: 92,
            half_width: 1,
            owner: [1, 2, 0x1000_0002],
            style: 10,
            generation: 1,
            flags: 1,
        });
        let b = table_bytes(&t);
        assert_eq!(b.chart.len(), CHART_ROW_BYTES);
        assert_eq!(&b.chart[..8], &[0xA8, 0xFF, 0xFF, 0xFF, 112, 0, 0, 0]);
        assert_eq!(&b.chart[8..12], &3073u32.to_le_bytes());
        assert_eq!(&b.chart[28..], &[0; 4]);
        assert_eq!(b.frame.len(), FRAME_ROW_BYTES);
        assert_eq!(&b.frame[16..20], &[5, 0, 0, 0]);
        assert_eq!(b.stroke.len(), STROKE_ROW_BYTES);
        assert_eq!(&b.stroke[32..36], &0x1000_0002u32.to_le_bytes());
        assert_eq!(&b.stroke[36..48], &[10, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]);
        assert_eq!(
            b.style.len(),
            16 * 4,
            "styles 8, 9 and 10 came in 7.2; 12, 13 and 14 in 7.3; 15 in 7.4; 11 is unassigned"
        );
        for size in [CHART_ROW_BYTES, FRAME_ROW_BYTES, STROKE_ROW_BYTES] {
            assert_eq!(size % 16, 0);
        }
    }

    #[test]
    fn route_and_segment_rows_are_little_endian_in_field_order_and_old_rows_keep_their_sizes() {
        let mut t = Tables::empty();
        t.route.push(RouteRow {
            link: 7,
            fold: 1,
            size: 1296,
            ordered: 0,
            segment_first: 40,
            segment_count: 17,
            generation: 1,
            flags: 1,
        });
        t.segment.push(SegmentRow {
            chart: 3072,
            x0: -128,
            y0: 1216,
            x1: 4672,
            y1: 1216,
            route: 137,
            member: 3,
            kind: 0,
            legs: 2,
            style: 13,
            generation: 1,
            flags: 1,
        });
        let b = table_bytes(&t);
        assert_eq!(b.route.len(), ROUTE_ROW_BYTES);
        assert_eq!(&b.route[..12], &[7, 0, 0, 0, 1, 0, 0, 0, 0x10, 0x05, 0, 0]);
        assert_eq!(&b.route[16..24], &[40, 0, 0, 0, 17, 0, 0, 0]);
        assert_eq!(b.segment.len(), SEGMENT_ROW_BYTES);
        assert_eq!(&b.segment[4..8], &(-128i32).to_le_bytes());
        assert_eq!(&b.segment[20..24], &137u32.to_le_bytes());
        assert_eq!(&b.segment[36..48], &[13, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]);
        assert_eq!(
            [
                BODY_ROW_BYTES,
                CELL_ROW_BYTES,
                PORT_ROW_BYTES,
                LINK_ROW_BYTES,
                CHART_ROW_BYTES,
                FRAME_ROW_BYTES,
                STROKE_ROW_BYTES,
                ROUTE_ROW_BYTES,
                SEGMENT_ROW_BYTES
            ],
            [32, 48, 32, 32, 32, 32, 48, 32, 48]
        );
    }
}
