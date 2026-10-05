//! The cursor moved.

use super::{DRAG_PIXELS, Shell};

impl Shell {
    /// With the button down and the cursor 4 px or more from the press on
    /// either axis, pan by the whole pixels moved since the last position.
    pub fn drag_to(&mut self, x: i64, y: i64) -> Vec<String> {
        let Some((px, py)) = self.press else {
            return Vec::new();
        };
        if !self.dragging {
            if (x - px).abs() < DRAG_PIXELS && (y - py).abs() < DRAG_PIXELS {
                return Vec::new();
            }
            self.dragging = true;
        }
        let (dx, dy) = (x - self.last.0, y - self.last.1);
        self.last = (x, y);
        if (dx, dy) == (0, 0) {
            return Vec::new();
        }
        let panned = self.view.camera().pan(dx, dy);
        self.apply(panned)
    }
}
