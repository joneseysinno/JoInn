//! Move the pin to a pixel and the focus to the point under it: the camera's
//! only rounding.

use joinn_frame::Verdict;

use super::Camera;
use crate::refuse::refuse;

impl Camera {
    /// `focus ← floor₂₋₁₆(focus + (q − pin)/k)`, `pin ← q`. Nothing changes when
    /// `q` is already the pin. The picture moves by less than `k·2^-16` px.
    pub fn refocus(self, q: (i64, i64)) -> Verdict<Camera> {
        if q == self.pin {
            return Verdict::Ok(self);
        }
        let base = 256 + i128::from(self.zoom.step);
        let shift = 24 - self.zoom.level;
        let at = |focus: i64, q: i64, pin: i64| -> Option<i64> {
            let d = i128::from(q) - i128::from(pin);
            let moved = (d << shift).div_euclid(base);
            i64::try_from(i128::from(focus) + moved).ok()
        };
        let (Some(fx), Some(fy)) = (
            at(self.focus.0, q.0, self.pin.0),
            at(self.focus.1, q.1, self.pin.1),
        ) else {
            return refuse(format!(
                "zoom: refocus at pixel {} {} leaves the focus range; acceptance is a pixel whose point fits in 2^-16 layout units",
                q.0, q.1
            ));
        };
        Verdict::Ok(Camera {
            focus: (fx, fy),
            pin: q,
            ..self
        })
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use crate::camera::{Camera, ChartId, FOCUS_UNIT, LEVEL_MAX, LEVEL_MIN, Zoom};

    fn camera(level: i32, step: u32) -> Camera {
        Camera {
            zoom: Zoom { level, step },
            anchor: ChartId(0),
            focus: (2752 * FOCUS_UNIT + 12345, 800 * FOCUS_UNIT - 777),
            pin: (960, 540),
            width: 1920,
            height: 1080,
        }
    }

    #[test]
    fn refocus_moves_the_focus_by_less_than_one_focus_unit() {
        let mut checked = 0;
        for level in LEVEL_MIN..=LEVEL_MAX {
            for step in [0u32, 1, 95, 128, 255] {
                let c = camera(level, step);
                for q in [(0, 0), (1, 1), (961, 539), (1919, 1079), (-7, 3000)] {
                    let r = match c.refocus(q) {
                        Verdict::Ok(r) => r,
                        Verdict::Refused(e) => panic!("{}", e.reason),
                    };
                    assert_eq!(r.pin, q);
                    let base = 256 + i128::from(step);
                    let shift = 24 - level;
                    for (f, f2, d) in [
                        (c.focus.0, r.focus.0, q.0 - c.pin.0),
                        (c.focus.1, r.focus.1, q.1 - c.pin.1),
                    ] {
                        let ideal_times_base = i128::from(f) * base + (i128::from(d) << shift);
                        let new_times_base = i128::from(f2) * base;
                        assert!(new_times_base <= ideal_times_base, "{level} {step} {q:?}");
                        assert!(
                            ideal_times_base < new_times_base + base,
                            "{level} {step} {q:?}"
                        );
                    }
                    checked += 1;
                }
            }
        }
        assert_eq!(checked, 14 * 5 * 5);
    }

    #[test]
    fn refocus_at_the_pin_changes_nothing() {
        let c = camera(3, 17);
        assert_eq!(c.refocus(c.pin), Verdict::Ok(c));
    }
}
