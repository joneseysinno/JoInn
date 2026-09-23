//! Canonicalize rational to ℚ value.

use joinn_frame::{Frame, RatFrame, Term, Value, Verdict};
use num_rational::BigRational;

pub(crate) fn canon_rat(r: BigRational) -> Verdict<Value> {
    RatFrame::new().canonicalize(Term::Seq(vec![
        Term::Int(r.numer().clone()),
        Term::Int(r.denom().clone()),
    ]))
}
