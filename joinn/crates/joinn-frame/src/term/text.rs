//! `Term::text`.

use super::Term;

impl Term {
    /// Text term. Not yet NFC — the frame canonicalizes.
    pub fn text(s: impl Into<String>) -> Self {
        Term::Text(s.into())
    }
}
