//! Drop the last typed character.

use super::Desktop;

impl Desktop {
    /// Echo the buffer after dropping its last character.
    pub fn backspace(&mut self) -> Option<String> {
        self.selected.as_ref()?;
        self.buffer.pop();
        Some(format!("  typing: {}", self.buffer))
    }
}
