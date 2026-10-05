//! Keys: camera keys, or typing into the selection.

use winit::event::ElementState;
use winit::keyboard::{Key as WinitKey, NamedKey};

use super::ShellApp;
use crate::session::Key;

impl ShellApp {
    /// A press is named without winit's types and handed to the shell.
    pub(super) fn on_key(&mut self, state: ElementState, key: WinitKey) {
        if state != ElementState::Pressed {
            return;
        }
        let key = match key {
            WinitKey::Named(NamedKey::Escape) => Key::Escape,
            WinitKey::Named(NamedKey::Enter) => Key::Enter,
            WinitKey::Named(NamedKey::Backspace) => Key::Backspace,
            WinitKey::Named(NamedKey::ArrowLeft) => Key::Left,
            WinitKey::Named(NamedKey::ArrowRight) => Key::Right,
            WinitKey::Named(NamedKey::ArrowUp) => Key::Up,
            WinitKey::Named(NamedKey::ArrowDown) => Key::Down,
            WinitKey::Character(text) => Key::Text(text.to_string()),
            _ => return,
        };
        let lines = self.shell.key(key);
        self.after_input(lines);
    }
}
