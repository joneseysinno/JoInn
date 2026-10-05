//! A left press.

use super::Shell;

impl Shell {
    /// Remember where, so the release can tell a click from a drag.
    pub fn press(&mut self, x: i64, y: i64) {
        self.press = Some((x, y));
        self.last = (x, y);
        self.dragging = false;
    }
}
