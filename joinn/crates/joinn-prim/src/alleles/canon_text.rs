//! Canonicalize string to Text value.

use joinn_frame::{Frame, Term, TextFrame, Value, Verdict};

pub(crate) fn canon_text(s: String) -> Verdict<Value> {
    TextFrame::new().canonicalize(Term::Text(s))
}
