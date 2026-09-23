//! Canonicalize a text literal for cell fixtures.

use joinn_frame::Value;

/// Canonicalize a text literal for cell fixtures.
pub(in crate::model) fn generate_text(s: &str) -> Value {
    let text = joinn_frame::TextFrame::new();
    match joinn_frame::Frame::canonicalize(&text, joinn_frame::Term::text(s)) {
        joinn_frame::Verdict::Ok(v) => v,
        joinn_frame::Verdict::Refused(_) => joinn_frame::Frame::generate(&text, 0, 1),
    }
}
