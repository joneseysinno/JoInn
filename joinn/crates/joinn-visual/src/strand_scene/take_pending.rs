//! Hand the rows changed since the last draw to the renderer.

use super::StrandScene;
use crate::tables::Delta;

impl StrandScene {
    /// `None`: nothing to draw (V12).
    pub fn take_pending(&mut self) -> Option<Delta> {
        if self.pending.is_empty() {
            return None;
        }
        let rows = std::mem::take(&mut self.pending).into_iter().collect();
        Some(Delta { rows })
    }
}
