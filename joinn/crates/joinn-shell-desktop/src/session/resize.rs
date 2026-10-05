//! A new viewport size.

use joinn_frame::Verdict;

use super::Shell;

impl Shell {
    /// The first size frames the view. Later sizes keep zoom, focus and pin,
    /// so nothing jumps; the centre pixel may fall in another chart and rebase.
    pub fn resize(&mut self, width: u32, height: u32) -> Vec<String> {
        if self.framed {
            let sized = self.view.camera().resize(width, height);
            return self.apply(sized);
        }
        match self.view.home(width, height) {
            Verdict::Ok(home) => {
                self.framed = true;
                self.apply(home)
            }
            Verdict::Refused(r) => vec![format!("refused: {}", r.reason)],
        }
    }
}
