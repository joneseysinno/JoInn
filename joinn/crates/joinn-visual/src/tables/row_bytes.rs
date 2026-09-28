//! One row's bytes: its fields little-endian in field order, padded with zeros.

use super::{
    BODY_ROW_BYTES, CELL_ROW_BYTES, INCIDENCE_BYTES, LINK_ROW_BYTES, PORT_ROW_BYTES, Table, Tables,
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
    };
    let mut out: Vec<u8> = words.into_iter().flatten().collect();
    out.resize(size, 0);
    Some(out)
}
