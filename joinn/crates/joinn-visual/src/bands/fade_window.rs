//! Whether a size sits in a crossfade window, and which.

use super::THRESHOLDS;
use crate::camera::Zoom;

/// `Some(T)` when `T ≤ s < 1.2·T` for a threshold `T`: the lower band fades
/// out and the higher fades in. Tested as `5·s` against `5·T` and `6·T`, in
/// integers.
pub fn fade_window(zoom: Zoom, size: i64) -> Option<i64> {
    let a = ((256 + i128::from(zoom.step)) * i128::from(size)) << zoom.level.max(0);
    let b = 256i128 << (-zoom.level).max(0);
    THRESHOLDS.into_iter().find(|t| {
        let t = i128::from(*t);
        5 * a >= 5 * t * b && 5 * a < 6 * t * b
    })
}

#[cfg(test)]
mod tests {
    use super::fade_window;
    use crate::camera::Zoom;

    #[test]
    fn a_window_runs_from_t_up_to_but_not_including_1_2_t() {
        assert_eq!(fade_window(Zoom { level: 3, step: 0 }, 30), Some(240));
        assert_eq!(fade_window(Zoom { level: 3, step: 0 }, 36), None);
        assert_eq!(fade_window(Zoom { level: 3, step: 0 }, 35), Some(240));
        assert_eq!(fade_window(Zoom { level: 0, step: 0 }, 4), Some(4));
        assert_eq!(fade_window(Zoom { level: 0, step: 0 }, 3), None);
        assert_eq!(fade_window(Zoom { level: -3, step: 0 }, 304), Some(32));
        assert_eq!(
            fade_window(
                Zoom {
                    level: -2,
                    step: 95
                },
                40
            ),
            None
        );
    }
}
