//! Frame a rectangle: the largest zoom that fits it with 16 px to spare a side.

use super::{Camera, ChartId, FOCUS_UNIT, LEVEL_MAX, LEVEL_MIN, STEPS, Zoom};
use crate::layout::Rect;

impl Camera {
    /// The largest zoom with `k·w ≤ width − 32` and `k·h ≤ height − 32`
    /// (level −4 step 0 when none fits); the focus is the rectangle's centre in
    /// `anchor`, pinned to `(width div 2, height div 2)`.
    pub fn frame(rect: Rect, anchor: ChartId, width: u32, height: u32) -> Camera {
        let room_w = i128::from(width) - 32;
        let room_h = i128::from(height) - 32;
        let fits = |zoom: Zoom, size: i64, room: i128| -> bool {
            let base = 256 + i128::from(zoom.step);
            let lhs = (base * i128::from(size)) << zoom.level.max(0);
            let rhs = (256 * room) << (-zoom.level).max(0);
            lhs <= rhs
        };
        let mut chosen = Zoom {
            level: LEVEL_MIN,
            step: 0,
        };
        'search: for level in (LEVEL_MIN..=LEVEL_MAX).rev() {
            for step in (0..STEPS).rev() {
                let zoom = Zoom { level, step };
                if fits(zoom, rect.w, room_w) && fits(zoom, rect.h, room_h) {
                    chosen = zoom;
                    break 'search;
                }
            }
        }
        let half = FOCUS_UNIT / 2;
        Camera {
            zoom: chosen,
            anchor,
            focus: ((2 * rect.x + rect.w) * half, (2 * rect.y + rect.h) * half),
            pin: (i64::from(width / 2), i64::from(height / 2)),
            width,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::camera::{Camera, ChartId, FOCUS_UNIT, Zoom};
    use crate::layout::Rect;

    fn grove() -> Rect {
        Rect {
            x: 0,
            y: 0,
            w: 5504,
            h: 1600,
        }
    }

    #[test]
    fn the_grove_frames_two_levels_down_at_step_95_at_1920_by_1080() {
        let c = Camera::frame(grove(), ChartId(0), 1920, 1080);
        assert_eq!(
            c.zoom,
            Zoom {
                level: -2,
                step: 95
            }
        );
        assert_eq!(c.zoom.k(), (351, 1024));
        assert_eq!(c.focus, (2752 * FOCUS_UNIT, 800 * FOCUS_UNIT));
        assert_eq!(c.pin, (960, 540));
        let (num, den) = c.zoom.k();
        assert!(num * 5504 <= 1888 * den);
        let (num, den) = Zoom {
            level: -2,
            step: 96,
        }
        .k();
        assert!(num * 5504 > 1888 * den, "step 96 gives 1892");
    }

    #[test]
    fn a_viewport_nothing_fits_gets_the_least_zoom() {
        let c = Camera::frame(grove(), ChartId(0), 40, 40);
        assert_eq!(c.zoom, Zoom { level: -4, step: 0 });
        assert_eq!(c.pin, (20, 20));
    }

    #[test]
    fn a_small_rectangle_frames_at_the_deepest_zoom_that_fits() {
        let c = Camera::frame(
            Rect {
                x: 88,
                y: 112,
                w: 2,
                h: 1,
            },
            ChartId(7),
            1920,
            1080,
        );
        assert_eq!(
            c.zoom,
            Zoom {
                level: 9,
                step: 216
            }
        );
        assert_eq!(c.anchor, ChartId(7));
        assert_eq!(
            c.focus,
            (89 * FOCUS_UNIT, 112 * FOCUS_UNIT + FOCUS_UNIT / 2)
        );
    }
}
