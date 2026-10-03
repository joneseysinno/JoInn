//! Pan by whole pixels.

use super::Camera;

impl Camera {
    /// Only the pin moves; the focus and zoom stay.
    pub fn pan(self, dx: i64, dy: i64) -> Camera {
        Camera {
            pin: (self.pin.0 + dx, self.pin.1 + dy),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::camera::{Camera, ChartId, PIXEL_UNIT, Zoom};

    #[test]
    fn a_pan_moves_every_point_by_whole_pixels() {
        let c = Camera {
            zoom: Zoom { level: 3, step: 9 },
            anchor: ChartId(0),
            focus: (1000, -2000),
            pin: (10, 20),
            width: 640,
            height: 360,
        };
        let p = c.pan(-7, 64);
        assert_eq!((p.pin, p.focus, p.zoom), ((3, 84), c.focus, c.zoom));
        let (x0, y0) = c.pixel_of((5555, 6666));
        let (x1, y1) = p.pixel_of((5555, 6666));
        assert_eq!((x1 - x0, y1 - y0), (-7 * PIXEL_UNIT, 64 * PIXEL_UNIT));
    }
}
