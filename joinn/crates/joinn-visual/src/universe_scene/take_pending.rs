//! Hand the rows changed since the last draw to the renderer.

use super::UniverseScene;
use crate::tables::Delta;

impl UniverseScene {
    /// `None`: nothing to draw (V12, V147).
    pub fn take_pending(&mut self) -> Option<Delta> {
        if self.pending.is_empty() {
            return None;
        }
        let rows = std::mem::take(&mut self.pending).into_iter().collect();
        Some(Delta { rows })
    }
}
