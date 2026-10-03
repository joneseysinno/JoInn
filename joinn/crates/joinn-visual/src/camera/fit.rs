//! Fit a layout's surface to a viewport.

use super::FitCamera;
use crate::layout::Layout;

/// `k` is the largest multiple of 4 with `W·k ≤ width − 32` and `H·k ≤ height − 32`,
/// and at least 4. The surface is centred, rounding down.
pub fn fit(layout: &Layout, width: u32, height: u32) -> FitCamera {
    let (w, h) = (layout.surface.w, layout.surface.h);
    let (vw, vh) = (i64::from(width), i64::from(height));
    let most = |room: i64, size: i64| {
        if size > 0 {
            room.div_euclid(size)
        } else {
            i64::MAX
        }
    };
    let k = most(vw - 32, w).min(most(vh - 32, h));
    let k = (k.div_euclid(4) * 4).max(4);
    FitCamera {
        k,
        ox: (vw - w * k).div_euclid(2),
        oy: (vh - h * k).div_euclid(2),
        width,
        height,
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use super::fit;
    use crate::camera::{STANDARD_VIEWPORTS, print_camera};
    use crate::fixtures::calculator;
    use crate::layout::layout;

    #[test]
    fn the_calculator_fits_the_four_standard_viewports() {
        let (body, cells) = calculator();
        let l = match layout(&body, &cells) {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let got: Vec<String> = STANDARD_VIEWPORTS
            .iter()
            .map(|(w, h)| print_camera(&fit(&l, *w, *h)))
            .collect();
        assert_eq!(
            got,
            [
                "camera 640x360: k 12, origin 80 36",
                "camera 1000x777: k 24, origin 20 100",
                "camera 1280x720: k 28, origin 80 24",
                "camera 1920x1080: k 40, origin 160 60",
            ]
        );
    }

    #[test]
    fn a_viewport_too_small_still_gets_k_4() {
        let (body, cells) = calculator();
        let l = match layout(&body, &cells) {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let c = fit(&l, 100, 50);
        assert_eq!((c.k, c.ox, c.oy), (4, -30, -23));
    }
}
