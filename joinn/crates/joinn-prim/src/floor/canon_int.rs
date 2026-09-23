//! Canonical ℤ value from an i64.

use joinn_frame::{Frame, IntFrame, Term, Value, Verdict};

pub(in crate::floor) fn canon_int(n: i64) -> Verdict<Value> {
    IntFrame::new().canonicalize(Term::int(n))
}
