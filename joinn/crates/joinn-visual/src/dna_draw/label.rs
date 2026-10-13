//! A name in the stroke font, as pieces.

use joinn_frame::Verdict;

use super::Piece;
use crate::font::{ADVANCE, GRID_W, LABEL_SIZE, text_strokes};
use crate::tables::STYLE_TEXT;

/// `text` at label size, centred on `centre_x`, its cap top at `top`.
pub(crate) fn label(text: &str, centre_x: i64, top: i64, owner: [u32; 3]) -> Verdict<Vec<Piece>> {
    let glyphs = text.chars().count() as i64;
    let width = ((glyphs - 1).max(0) * ADVANCE + GRID_W) * LABEL_SIZE.unit;
    match text_strokes(text, (centre_x - width / 2, top), LABEL_SIZE) {
        Verdict::Ok(strokes) => Verdict::Ok(
            strokes
                .into_iter()
                .map(|s| Piece {
                    a: (s.x0, s.y0),
                    b: (s.x1, s.y1),
                    half: s.half_width,
                    style: STYLE_TEXT,
                    owner,
                })
                .collect(),
        ),
        Verdict::Refused(r) => Verdict::Refused(r),
    }
}
