//! Canonicalize bigint to ℤ value.

use joinn_frame::{Frame, IntFrame, Term, Value, Verdict};
use num_bigint::BigInt;

pub(crate) fn canon_int(n: BigInt) -> Verdict<Value> {
    IntFrame::new().canonicalize(Term::Int(n))
}
