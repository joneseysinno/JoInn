//! A left release.

use super::Shell;

impl Shell {
    /// The end of a drag, or a click at the press pixel when the cursor moved
    /// less than 4 px. A click owes a tick, which confirms it on the GPU.
    pub fn release(&mut self, x: i64, y: i64) -> Vec<String> {
        let lines = self.drag_to(x, y);
        let pressed = self.press.take();
        if std::mem::replace(&mut self.dragging, false) {
            return lines;
        }
        let Some((px, py)) = pressed else {
            return lines;
        };
        let (Ok(px), Ok(py)) = (u32::try_from(px), u32::try_from(py)) else {
            return lines;
        };
        self.dirty = true;
        self.view.click(px, py)
    }
}
