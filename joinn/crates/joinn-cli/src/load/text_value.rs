//! Canonicalize a text string as a `Value`.

use joinn_frame::{Frame, Term, TextFrame, Value, Verdict};

/// Canonicalize a text string as a `Value`.
pub(crate) fn text_value(s: &str) -> Result<Value, String> {
    match TextFrame::new().canonicalize(Term::text(s)) {
        Verdict::Ok(v) => Ok(v),
        Verdict::Refused(r) => Err(r.reason),
    }
}
