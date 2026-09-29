//! Echo one typed character into the selected port's buffer.

use super::Desktop;

impl Desktop {
    /// `  typing: 2` once `2` is in the buffer. Nothing is echoed with no selection.
    pub fn type_char(&mut self, text: &str) -> Option<String> {
        if self.selected.is_none() || text.is_empty() {
            return None;
        }
        self.buffer.push_str(text);
        Some(format!("  typing: {}", self.buffer))
    }
}
