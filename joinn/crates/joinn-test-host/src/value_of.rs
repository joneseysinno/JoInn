//! Canonicalize a raw term into a value.

use joinn_frame::{Frame, IntFrame, RatFrame, Term, TextFrame, Value, Verdict};

/// Canonicalize a raw term into a value.
pub(crate) fn value_of(term: Term) -> Verdict<Value> {
    match &term {
        Term::Text(_) => TextFrame::new().canonicalize(term),
        Term::Int(_) => IntFrame::new().canonicalize(term),
        Term::Seq(_) => RatFrame::new().canonicalize(term),
    }
}
