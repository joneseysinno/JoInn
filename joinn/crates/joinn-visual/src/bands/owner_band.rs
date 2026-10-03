//! The owner band: the higher band once past the middle of its fade window.

use super::{Band, THRESHOLDS};
use crate::camera::Zoom;

/// The higher band owns exactly when `10·s ≥ 11·T`. With
/// `s = (256 + step)·size·2^level / 256` the test is
/// `10·(256 + step)·size·2^level ≥ 11·T·256`, each power of two moved to the
/// side that keeps both whole.
pub fn owner_band(zoom: Zoom, size: i64) -> Band {
    let a = ((256 + i128::from(zoom.step)) * i128::from(size)) << zoom.level.max(0);
    let b = 256i128 << (-zoom.level).max(0);
    let past = |t: i64| 10 * a >= 11 * i128::from(t) * b;
    match THRESHOLDS {
        [_, _, full] if past(full) => Band::Full,
        [_, summary, _] if past(summary) => Band::Summary,
        [glyph, _, _] if past(glyph) => Band::Glyph,
        _ => Band::Dot,
    }
}

#[cfg(test)]
mod tests {
    use super::owner_band;
    use crate::bands::Band;
    use crate::camera::Zoom;

    #[test]
    fn at_10s_equal_to_11t_the_higher_band_owns_and_one_step_below_the_lower() {
        assert_eq!(owner_band(Zoom { level: 3, step: 0 }, 33), Band::Full);
        assert_eq!(
            owner_band(
                Zoom {
                    level: 2,
                    step: 255
                },
                33
            ),
            Band::Summary
        );
    }

    #[test]
    fn the_glyph_and_summary_thresholds_flip_between_one_step_and_the_next() {
        let z = |level, step| Zoom { level, step };
        assert_eq!(owner_band(z(-4, 194), 40), Band::Dot);
        assert_eq!(owner_band(z(-4, 195), 40), Band::Glyph);
        assert_eq!(owner_band(z(0, 194), 20), Band::Glyph);
        assert_eq!(owner_band(z(0, 195), 20), Band::Summary);
    }

    #[test]
    fn the_plan_s_sizes_band_as_predicted() {
        let z = |level| Zoom { level, step: 0 };
        assert_eq!(owner_band(z(-3), 40), Band::Glyph);
        assert_eq!(owner_band(z(-3), 20), Band::Dot);
        assert_eq!(owner_band(z(0), 40), Band::Summary);
        assert_eq!(owner_band(z(0), 20), Band::Glyph);
        assert_eq!(owner_band(z(3), 40), Band::Full);
        assert_eq!(owner_band(z(3), 20), Band::Summary);
        assert_eq!(owner_band(z(-4), 304), Band::Glyph);
        assert_eq!(owner_band(z(-3), 304), Band::Summary);
    }
}
