//! Write a picture's rows.

use joinn_frame::Verdict;

use super::Piece;
use crate::layout::SURFACE_RADIUS;
use crate::refuse::refuse;
use crate::tables::{BodyRow, CHART_BODY, ChartRow, LIVE, Row, RowWrite, StrokeRow, Table, Tables};

/// Body 0 and chart 0 (a body chart at the origin) cover `extent`, given in
/// sixteenths and rounded up to whole layout units; stroke slot `i` is piece
/// `i`. Only rows whose bytes changed are named; stroke rows past the new end
/// are dropped and named, so the tables equal a fresh picture's.
pub(crate) fn write_pieces(
    tables: &mut Tables,
    extent: (i64, i64),
    pieces: &[Piece],
) -> Verdict<Vec<RowWrite>> {
    let units = |v: i64| (v + 15).div_euclid(16);
    let (w, h) = (units(extent.0), units(extent.1));
    let (Ok(wi), Ok(hi), Ok(size), Ok(radius)) = (
        i32::try_from(w),
        i32::try_from(h),
        u32::try_from(w.max(h)),
        u32::try_from(SURFACE_RADIUS),
    ) else {
        return refuse(format!(
            "picture: {w} x {h} does not fit a table field; acceptance is a picture within 32 bits"
        ));
    };
    let mut rows = vec![
        (
            0u32,
            Row::Body(BodyRow {
                x: 0,
                y: 0,
                w: wi,
                h: hi,
                radius,
                generation: 1,
                flags: LIVE,
            }),
        ),
        (
            0u32,
            Row::Chart(ChartRow {
                origin_x: 0,
                origin_y: 0,
                parent: 0,
                kind: CHART_BODY,
                size,
                generation: 1,
                flags: LIVE,
            }),
        ),
    ];
    for (slot, p) in (0u32..).zip(pieces) {
        let (Ok(x0), Ok(y0), Ok(x1), Ok(y1), Ok(half_width)) = (
            i32::try_from(p.a.0),
            i32::try_from(p.a.1),
            i32::try_from(p.b.0),
            i32::try_from(p.b.1),
            u32::try_from(p.half),
        ) else {
            return refuse(format!(
                "picture: stroke {slot} does not fit a table field; acceptance is a picture within 32 bits"
            ));
        };
        rows.push((
            slot,
            Row::Stroke(StrokeRow {
                chart: 0,
                x0,
                y0,
                x1,
                y1,
                half_width,
                owner: p.owner,
                style: p.style,
                generation: 1,
                flags: LIVE,
            }),
        ));
    }
    let mut changed: Vec<RowWrite> = rows
        .into_iter()
        .filter_map(|(slot, row)| tables.put(slot, row).map(|table| RowWrite { table, slot }))
        .collect();
    let len = pieces.len();
    for slot in len..tables.stroke.len() {
        let Ok(slot) = u32::try_from(slot) else {
            return refuse(
                "picture: a stroke slot does not fit 32 bits; acceptance is a smaller picture",
            );
        };
        changed.push(RowWrite {
            table: Table::Stroke,
            slot,
        });
    }
    tables.stroke.truncate(len);
    changed.sort();
    changed.dedup();
    Verdict::Ok(changed)
}
