//! Zoom by notches about one pixel.

use joinn_frame::Verdict;

use super::Camera;

impl Camera {
    /// Refocus on `q` (only when it is not already the pin), then notch the
    /// zoom. Zoom itself never changes focus or pin.
    pub fn zoom_about(self, q: (i64, i64), notches: i32) -> Verdict<Camera> {
        let c = match self.refocus(q) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        match c.zoom.notch(notches) {
            Verdict::Ok(zoom) => Verdict::Ok(Camera { zoom, ..c }),
            Verdict::Refused(r) => Verdict::Refused(r),
        }
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use crate::camera::{Camera, ChartId, FOCUS_UNIT, Zoom};

    fn ok(v: Verdict<Camera>) -> Camera {
        match v {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    fn start() -> Camera {
        Camera {
            zoom: Zoom {
                level: -2,
                step: 95,
            },
            anchor: ChartId(0),
            focus: (2752 * FOCUS_UNIT, 800 * FOCUS_UNIT),
            pin: (960, 540),
            width: 1920,
            height: 1080,
        }
    }

    #[test]
    fn eight_in_then_eight_out_about_one_pixel_returns_the_same_camera() {
        for q in [(960, 540), (123, 77), (1919, 0)] {
            let pinned = ok(start().refocus(q));
            for from in [start(), pinned] {
                let mut c = from;
                for _ in 0..8 {
                    c = ok(c.zoom_about(q, 1));
                }
                assert_eq!(c.focus, pinned.focus);
                assert_eq!(c.pin, q);
                for _ in 0..8 {
                    c = ok(c.zoom_about(q, -1));
                }
                assert_eq!(c, pinned, "{q:?}");
            }
        }
    }

    #[test]
    fn zoom_about_the_pin_changes_only_the_zoom() {
        let c = ok(start().zoom_about((960, 540), 3));
        assert_eq!(c.focus, start().focus);
        assert_eq!(c.pin, start().pin);
        assert_eq!(
            c.zoom,
            Zoom {
                level: -2,
                step: 191
            }
        );
    }

    #[test]
    fn zooming_past_level_9_is_refused() {
        let mut c = start();
        c.zoom = Zoom {
            level: 9,
            step: 255,
        };
        let Verdict::Refused(r) = c.zoom_about((960, 540), 1) else {
            panic!("level 10 must be refused");
        };
        assert_eq!(
            r.reason,
            "zoom: level 10 is outside −4 … 9; acceptance is a level from −4 to 9"
        );
    }
}
