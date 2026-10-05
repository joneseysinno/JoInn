//! A left press at the last cursor position.

use super::ShellApp;
use super::pixel::pixel;

impl ShellApp {
    /// The shell remembers the pixel; the release decides click or drag.
    pub(super) fn on_press(&mut self) {
        if let Some((x, y)) = pixel(self.cursor) {
            self.shell.press(x, y);
        }
    }
}
