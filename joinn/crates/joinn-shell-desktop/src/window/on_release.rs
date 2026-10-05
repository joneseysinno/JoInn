//! A left release at the last cursor position.

use super::ShellApp;
use super::pixel::pixel;

impl ShellApp {
    /// A click prints the CPU pick; the redraw confirms it on the GPU.
    pub(super) fn on_release(&mut self) {
        if let Some((x, y)) = pixel(self.cursor) {
            let lines = self.shell.release(x, y);
            self.after_input(lines);
        }
    }
}
