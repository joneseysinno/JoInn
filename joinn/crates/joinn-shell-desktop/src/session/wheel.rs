//! The wheel: zoom about the pixel under the cursor.

use joinn_frame::Verdict;

use super::Shell;

impl Shell {
    /// `notches` in (positive) or out about pixel `(x, y)`. Past level −4 or 9
    /// the zoom is refused and nothing changes.
    pub fn wheel(&mut self, x: i64, y: i64, notches: i32) -> Vec<String> {
        if notches == 0 {
            return Vec::new();
        }
        match self.view.camera().zoom_about((x, y), notches) {
            Verdict::Ok(camera) => self.apply(camera),
            Verdict::Refused(r) => vec![format!("refused: {}", r.reason)],
        }
    }
}
