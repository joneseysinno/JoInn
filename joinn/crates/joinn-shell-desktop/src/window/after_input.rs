//! Print what an input printed, and redraw when it changed something.

use super::ShellApp;
use super::emit::emit;

impl ShellApp {
    /// Each line, then one redraw request when a tick is owed.
    pub(super) fn after_input(&self, lines: Vec<String>) {
        for line in lines {
            emit(&line);
        }
        if self.shell.dirty() {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }
}
