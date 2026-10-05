//! The cursor moved.

use super::ShellApp;
use super::pixel::pixel;

impl ShellApp {
    /// Remember the position; with the button down, the shell may pan.
    pub(super) fn on_moved(&mut self, x: f64, y: f64) {
        self.cursor = Some((x, y));
        if let Some((x, y)) = pixel(self.cursor) {
            let lines = self.shell.drag_to(x, y);
            self.after_input(lines);
        }
    }
}
