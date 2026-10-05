//! A key.

use super::{ARROW_PIXELS, Key, NOTHING_SENT, Shell};

impl Shell {
    /// `+` and `-` zoom one notch about the centre pixel and `F` frames, unless
    /// a port is selected, when text types into it. An arrow moves the camera
    /// 64 px, so the picture moves the other way. Enter owes a tick when it ran.
    pub fn key(&mut self, key: Key) -> Vec<String> {
        let c = self.view.camera();
        let (cx, cy) = (i64::from(c.width / 2), i64::from(c.height / 2));
        let typing = self.view.typing();
        match key {
            Key::Text(t) if !typing && t == "+" => self.wheel(cx, cy, 1),
            Key::Text(t) if !typing && t == "-" => self.wheel(cx, cy, -1),
            Key::Text(t) if !typing && (t == "f" || t == "F") => self.frame(),
            Key::Text(t) => self.view.type_char(&t).into_iter().collect(),
            Key::Left => self.apply(c.pan(ARROW_PIXELS, 0)),
            Key::Right => self.apply(c.pan(-ARROW_PIXELS, 0)),
            Key::Up => self.apply(c.pan(0, ARROW_PIXELS)),
            Key::Down => self.apply(c.pan(0, -ARROW_PIXELS)),
            Key::Enter => {
                let lines = self.view.enter();
                if lines.iter().any(|line| line != NOTHING_SENT) {
                    self.dirty = true;
                }
                lines
            }
            Key::Backspace => self.view.backspace().into_iter().collect(),
            Key::Escape => {
                self.view.escape();
                Vec::new()
            }
        }
    }
}
