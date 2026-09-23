//! Bounded integer sample.

use joinn_frame::{Frame, IntFrame, Term, Value, Verdict};
use std::num::NonZeroU32;

pub(in crate::seals) fn bounded_int(seed: u64, i: u32, bound: NonZeroU32) -> Value {
    let b = bound.get();
    let span = i64::from(b) * 2 + 1;
    let n = (seed.wrapping_add(u64::from(i)) as i64).rem_euclid(span) - i64::from(b);
    match IntFrame::new().canonicalize(Term::int(n)) {
        Verdict::Ok(v) => v,
        Verdict::Refused(_) => IntFrame::new().generate(0, 1),
    }
}
