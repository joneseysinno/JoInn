//! Zoom the fold to another floor.

use joinn_frame::Verdict;

use super::FoldScene;

impl FoldScene {
    /// Floor 0 is every segment; the order's own number is its top floor, a
    /// single node. Opened genes stay open.
    pub fn set_floor(&mut self, floor: u32) -> Verdict<()> {
        let opened = self.opened.clone();
        self.redraw(floor, opened)
    }
}
