//! Keys: type into the selection, Enter runs, Escape clears.

use winit::event::ElementState;
use winit::keyboard::{Key, NamedKey};

use super::ShellApp;
use super::emit::emit;
use crate::session::NOTHING_SENT;

impl ShellApp {
    /// A character echoes. Enter runs the body and redraws. Moving nothing else does not.
    pub(super) fn on_key(&mut self, state: ElementState, key: Key) {
        if state != ElementState::Pressed {
            return;
        }
        match key {
            Key::Named(NamedKey::Escape) => self.desktop.escape(),
            Key::Named(NamedKey::Enter) => {
                let lines = self.desktop.enter();
                let ran = lines.iter().any(|line| line != NOTHING_SENT);
                for line in lines {
                    emit(&line);
                }
                if ran {
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }
            Key::Named(NamedKey::Backspace) => {
                if let Some(line) = self.desktop.backspace() {
                    emit(&line);
                }
            }
            Key::Character(text) => {
                if let Some(line) = self.desktop.type_char(text.as_str()) {
                    emit(&line);
                }
            }
            _ => {}
        }
    }
}
