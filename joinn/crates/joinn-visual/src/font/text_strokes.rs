//! Lay a run of text out as placed strokes.

use joinn_frame::Verdict;

use super::{ADVANCE, TextSize, TextStroke, glyph};

/// The strokes of `text` with its first glyph's grid origin (cap top-left) at
/// `origin`, in sixteenths. Glyph `i` starts `i·ADVANCE` grid units right. A
/// character outside the stroke set refuses the whole run.
pub fn text_strokes(text: &str, origin: (i64, i64), size: TextSize) -> Verdict<Vec<TextStroke>> {
    let mut out = Vec::new();
    for (i, c) in (0i64..).zip(text.chars()) {
        let strokes = match glyph(c) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let left = origin.0 + i * ADVANCE * size.unit;
        out.extend(strokes.into_iter().map(|s| TextStroke {
            x0: left + s.x0 * size.unit,
            y0: origin.1 + s.y0 * size.unit,
            x1: left + s.x1 * size.unit,
            y1: origin.1 + s.y1 * size.unit,
            half_width: size.half_width,
        }));
    }
    Verdict::Ok(out)
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use super::text_strokes;
    use crate::font::LABEL_SIZE;

    #[test]
    fn the_second_glyph_starts_one_advance_right() {
        let Verdict::Ok(one) = text_strokes("1", (0, 0), LABEL_SIZE) else {
            panic!("1 has a glyph");
        };
        let Verdict::Ok(two) = text_strokes(" 1", (0, 0), LABEL_SIZE) else {
            panic!("' 1' has glyphs");
        };
        assert_eq!(one.len(), two.len());
        for (a, b) in one.iter().zip(&two) {
            assert_eq!(b.x0 - a.x0, 16);
            assert_eq!(b.y0, a.y0);
            assert_eq!(b.half_width, 1);
        }
    }

    #[test]
    fn one_unknown_character_refuses_the_run() {
        match text_strokes("ab!", (0, 0), LABEL_SIZE) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "text: '!' has no glyph; acceptance is a character from the stroke set"
            ),
            Verdict::Ok(_) => panic!("! has no glyph"),
        }
    }
}
