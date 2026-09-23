//! Canonicalize an integer literal for cell fixtures.

use joinn_frame::Value;

/// Canonicalize an integer literal for cell fixtures.
pub(in crate::model) fn generate_int(n: i64) -> Value {
    let int = joinn_frame::IntFrame::new();
    match joinn_frame::Frame::canonicalize(&int, joinn_frame::Term::int(n)) {
        joinn_frame::Verdict::Ok(v) => v,
        joinn_frame::Verdict::Refused(_) => joinn_frame::Frame::generate(&int, 0, 1),
    }
}
