//! A left press at the last cursor position.

use super::ShellApp;
use super::emit::emit;

impl ShellApp {
    /// The cursor is physical pixels. Floor them, print the CPU pick, and redraw.
    pub(super) fn on_click(&mut self) {
        let Some((x, y)) = self.cursor else {
            return;
        };
        let floor = |px: f64| -> Option<u32> {
            if !(0.0..f64::from(u32::MAX)).contains(&px) {
                return None;
            }
            let whole = px.floor();
            u32::try_from(whole as u64).ok()
        };
        let (Some(x), Some(y)) = (floor(x), floor(y)) else {
            return;
        };
        for line in self.desktop.click(x, y) {
            emit(&line);
        }
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
