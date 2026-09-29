//! Escape clears the selection.

use super::Desktop;

impl Desktop {
    /// The buffer goes with the selection.
    pub fn escape(&mut self) {
        self.selected = None;
        self.buffer.clear();
    }
}
