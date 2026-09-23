//! Bounded rational sample.

use joinn_frame::{Frame, IntFrame, RatFrame, Term, Value, Verdict};
use std::num::NonZeroU32;

use super::bounded_int::bounded_int;

pub(in crate::seals) fn bounded_rat(seed: u64, i: u32, bound: NonZeroU32) -> Value {
    let num = bounded_int(seed, i, bound);
    let den_span = i64::from(bound.get());
    let d = (seed.wrapping_add(u64::from(i).wrapping_mul(9)) as i64).rem_euclid(den_span) + 1;
    let den = match IntFrame::new().canonicalize(Term::int(d)) {
        Verdict::Ok(v) => v,
        Verdict::Refused(_) => match IntFrame::new().canonicalize(Term::int(1)) {
            Verdict::Ok(v) => v,
            Verdict::Refused(_) => return RatFrame::new().generate(seed, 4),
        },
    };
    let Term::Int(n) = num.term() else {
        return RatFrame::new().generate(seed, 4);
    };
    let Term::Int(dd) = den.term() else {
        return RatFrame::new().generate(seed, 4);
    };
    match RatFrame::new().canonicalize(Term::Seq(vec![Term::Int(n.clone()), Term::Int(dd.clone())]))
    {
        Verdict::Ok(v) => v,
        Verdict::Refused(_) => RatFrame::new().generate(seed, 4),
    }
}
