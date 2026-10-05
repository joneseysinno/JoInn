//! The mouse wheel.

use winit::event::MouseScrollDelta;

use super::ShellApp;
use super::pixel::pixel;

impl ShellApp {
    /// One notch per wheel line, or per touchpad gesture event, about the
    /// pixel under the cursor (the centre when the cursor is unknown).
    pub(super) fn on_wheel(&mut self, delta: MouseScrollDelta) {
        let notches = match delta {
            MouseScrollDelta::LineDelta(_, y) => y.round().clamp(-8.0, 8.0) as i32,
            MouseScrollDelta::PixelDelta(p) if p.y > 0.0 => 1,
            MouseScrollDelta::PixelDelta(p) if p.y < 0.0 => -1,
            MouseScrollDelta::PixelDelta(_) => 0,
        };
        let c = self.shell.camera();
        let (x, y) =
            pixel(self.cursor).unwrap_or((i64::from(c.width / 2), i64::from(c.height / 2)));
        let lines = self.shell.wheel(x, y, notches);
        self.after_input(lines);
    }
}
