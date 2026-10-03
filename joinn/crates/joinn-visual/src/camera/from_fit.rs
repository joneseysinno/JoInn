//! A Phase 6 camera, converted exactly.

use joinn_frame::Verdict;

use super::{Camera, ChartId, FitCamera, Zoom};

impl Camera {
    /// `(k, ox, oy)` is the zoom of whole `k`, focus 0 in the root chart, and pin
    /// `(ox, oy)`: layout point `u` lands on `o + k·u` in both.
    pub fn from_fit(fit: &FitCamera) -> Verdict<Camera> {
        let zoom = match Zoom::from_whole_k(fit.k) {
            Verdict::Ok(z) => z,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        Verdict::Ok(Camera {
            zoom,
            anchor: ChartId(0),
            focus: (0, 0),
            pin: (fit.ox, fit.oy),
            width: fit.width,
            height: fit.height,
        })
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use crate::camera::{Camera, FOCUS_UNIT, PIXEL_UNIT, STANDARD_VIEWPORTS, fit};
    use crate::fixtures::calculator;
    use crate::layout::layout;

    #[test]
    fn the_calculator_cameras_convert_and_land_every_corner_on_the_same_pixel() {
        let (body, cells) = calculator();
        let l = match layout(&body, &cells) {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        for (w, h) in STANDARD_VIEWPORTS {
            let f = fit(&l, w, h);
            let c = match Camera::from_fit(&f) {
                Verdict::Ok(c) => c,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            assert_eq!(c.zoom.whole_k(), Some(f.k));
            assert_eq!((c.width, c.height), (w, h));
            for (u, v) in [(0, 0), (40, 24), (24, 4), (36, 7)] {
                let want = (
                    i128::from(f.ox + f.k * u) * PIXEL_UNIT,
                    i128::from(f.oy + f.k * v) * PIXEL_UNIT,
                );
                assert_eq!(c.pixel_of((u * FOCUS_UNIT, v * FOCUS_UNIT)), want);
            }
        }
    }
}
