//! A system's or galaxy's title: its lens name.

use joinn_frame::Verdict;

use super::{GALAXY_TITLE_SIZE, SYSTEM_TITLE_SIZE, TextStroke, TitleOf, text_strokes};

/// The name at the frame's chart `(8, 4)` (system, cap 6) or `(16, 8)`
/// (galaxy, cap 16).
pub fn title(name: &str, of: TitleOf) -> Verdict<Vec<TextStroke>> {
    let (origin, size) = match of {
        TitleOf::System => ((16 * 8, 16 * 4), SYSTEM_TITLE_SIZE),
        TitleOf::Galaxy => ((16 * 16, 16 * 8), GALAXY_TITLE_SIZE),
    };
    text_strokes(name, origin, size)
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use super::title;
    use crate::font::TitleOf;

    #[test]
    fn a_system_title_sits_at_8_4_with_cap_6_and_a_galaxy_s_at_16_8_with_cap_16() {
        let Verdict::Ok(sys) = title("g0s00", TitleOf::System) else {
            panic!("g0s00 has glyphs");
        };
        let top = sys.iter().flat_map(|t| [t.y0, t.y1]).min();
        let left = sys.iter().flat_map(|t| [t.x0, t.x1]).min();
        assert_eq!((left, top), (Some(128), Some(64)));
        assert!(sys.iter().all(|t| t.half_width == 6));
        let Verdict::Ok(gal) = title("g0", TitleOf::Galaxy) else {
            panic!("g0 has glyphs");
        };
        let bottom = gal.iter().flat_map(|t| [t.y0, t.y1]).max();
        assert_eq!(bottom, Some(128 + 8 * 32));
        assert!(gal.iter().all(|t| t.half_width == 16));
    }
}
