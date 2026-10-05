//! `F`: frame the whole scene again.

use joinn_frame::Verdict;

use super::Shell;

impl Shell {
    /// The view's home camera at the current viewport size.
    pub fn frame(&mut self) -> Vec<String> {
        let c = self.view.camera();
        match self.view.home(c.width, c.height) {
            Verdict::Ok(home) => {
                self.framed = true;
                self.apply(home)
            }
            Verdict::Refused(r) => vec![format!("refused: {}", r.reason)],
        }
    }
}
