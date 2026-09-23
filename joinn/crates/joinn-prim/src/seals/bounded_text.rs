//! Bounded text sample.

use joinn_frame::{Frame, Term, TextFrame, Value, Verdict};
use std::num::NonZeroU32;

pub(in crate::seals) fn bounded_text(seed: u64, i: u32, bound: NonZeroU32) -> Value {
    let b = bound.get();
    let kind = seed.wrapping_add(u64::from(i)) % 10;
    let raw = match kind {
        0 => String::new(),
        1 => "0".into(),
        2 if b >= 3 => "007".into(),
        3 => "-1".into(),
        4 => "10".into(),
        5 => {
            let n = i64::from(i % b.saturating_add(1));
            n.to_string()
        }
        6 => " ".into(),
        7 => {
            let len = ((i % b) as usize).saturating_add(1).min(b as usize);
            "1".repeat(len.max(1))
        }
        8 => "-0".into(),
        _ => {
            let n = (seed.wrapping_add(u64::from(i)) as i64).rem_euclid(i64::from(b) * 2 + 1)
                - i64::from(b);
            n.to_string()
        }
    };
    match TextFrame::new().canonicalize(Term::text(&raw)) {
        Verdict::Ok(v) => v,
        Verdict::Refused(_) => TextFrame::new().generate(seed, 4),
    }
}
