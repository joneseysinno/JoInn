//! Name an ID texel of a system view.

use super::SystemScene;
use crate::pick::{FORCE_TAG, PORT_TAG, TAG_MASK};

impl SystemScene {
    /// `force count` for the lasso, `surface`, an instance (`count`,
    /// `numbers.3`) for a cell, `numbers.3@0` for a port, `background` for
    /// zero, and the raw texel for anything this scene did not draw.
    pub fn print_id(&self, id: [u32; 4]) -> String {
        let force = self.grown.force().map(|f| f.name.clone());
        let instance = |slot: u32| match slot {
            0 => None,
            1 => force.clone(),
            n => Some(self.grown.instance(n as usize - 2)),
        };
        let raw = || format!("[{} {} {:#x} {}]", id[0], id[1], id[2], id[3]);
        if id == [0; 4] {
            return "background".to_owned();
        }
        match (id[0], id[1], id[2] & TAG_MASK) {
            (1, 0, FORCE_TAG) => force.map_or_else(raw, |f| format!("force {f}")),
            (1, 0, 0) => "surface".to_owned(),
            (1, cell, 0) => instance(cell).unwrap_or_else(raw),
            (1, cell, PORT_TAG) => {
                instance(cell).map_or_else(raw, |i| format!("{i}@{}", id[2] & !TAG_MASK))
            }
            _ => raw(),
        }
    }
}
