//! A new viewport size.

use super::Camera;

impl Camera {
    /// Zoom, focus and pin do not change, so nothing jumps.
    pub fn resize(self, width: u32, height: u32) -> Camera {
        Camera {
            width,
            height,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::camera::{Camera, ChartId, Zoom};

    #[test]
    fn a_resize_keeps_every_point_on_its_pixel() {
        let c = Camera {
            zoom: Zoom {
                level: -1,
                step: 200,
            },
            anchor: ChartId(0),
            focus: (77, 88),
            pin: (960, 540),
            width: 1920,
            height: 1080,
        };
        let r = c.resize(1280, 720);
        assert_eq!((r.width, r.height), (1280, 720));
        assert_eq!(
            r.pixel_of((123_456, 654_321)),
            c.pixel_of((123_456, 654_321))
        );
    }
}
