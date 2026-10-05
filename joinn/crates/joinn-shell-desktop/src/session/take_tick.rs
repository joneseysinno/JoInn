//! The tick an input owed.

use super::{Shell, Tick};

impl Shell {
    /// Once per change: the zoom, the anchor and the rows written since the
    /// last tick. `None` when nothing changed: an idle window draws nothing
    /// (V147).
    pub fn take_tick(&mut self) -> Option<Tick> {
        if !self.dirty {
            return None;
        }
        self.dirty = false;
        Some(Tick {
            zoom: self.view.camera().zoom,
            anchor: self.view.anchor_name(),
            delta: self.view.take_pending(),
        })
    }
}
