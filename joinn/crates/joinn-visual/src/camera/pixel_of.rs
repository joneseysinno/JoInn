//! Where an anchor-chart point lands, exactly, in 2^-32 px.

use super::Camera;

impl Camera {
    /// The pixel of anchor-chart point `p` (in 2^-16 layout units) is
    /// `pin + k·(p − focus)`. Answered in units of 2^-32 px, which is exact for
    /// every level from −4 up: `(256 + step)·(p − focus)·2^(level + 8)`.
    pub fn pixel_of(&self, p: (i64, i64)) -> (i128, i128) {
        let base = 256 + i128::from(self.zoom.step);
        let shift = self.zoom.level + 8;
        let at = |pin: i64, p: i64, focus: i64| -> i128 {
            let offset = base * (i128::from(p) - i128::from(focus));
            let scaled = if shift >= 0 {
                offset << shift
            } else {
                offset >> -shift
            };
            (i128::from(pin) << 32) + scaled
        };
        (
            at(self.pin.0, p.0, self.focus.0),
            at(self.pin.1, p.1, self.focus.1),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::camera::{Camera, ChartId, FOCUS_UNIT, PIXEL_UNIT, Zoom};

    #[test]
    fn the_focus_lands_on_the_pin_and_one_unit_right_lands_k_pixels_right() {
        let c = Camera {
            zoom: Zoom {
                level: -2,
                step: 95,
            },
            anchor: ChartId(0),
            focus: (2752 * FOCUS_UNIT, 800 * FOCUS_UNIT),
            pin: (960, 540),
            width: 1920,
            height: 1080,
        };
        assert_eq!(c.pixel_of(c.focus), (960 * PIXEL_UNIT, 540 * PIXEL_UNIT));
        let (x, _) = c.pixel_of((2752 * FOCUS_UNIT + 1024 * FOCUS_UNIT, 800 * FOCUS_UNIT));
        assert_eq!(x, (960 + 351) * PIXEL_UNIT);
        let (x0, _) = c.pixel_of((0, 0));
        assert_eq!(x0 * 1024, 960 * PIXEL_UNIT * 1024 - 2752 * 351 * PIXEL_UNIT);
    }
}
