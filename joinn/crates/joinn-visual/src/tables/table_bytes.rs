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
    }
}

#[cfg(test)]
mod tests {
    use super::table_bytes;
    use crate::tables::{
        BODY_ROW_BYTES, BodyRow, CELL_ROW_BYTES, CellRow, LINK_ROW_BYTES, PORT_ROW_BYTES, Tables,
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
}
