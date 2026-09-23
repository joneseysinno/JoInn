//! Sample one port value.

use joinn_frame::{Frame, FrameId, FrameRef, IntFrame, RatFrame, Term, TextFrame, Value, Verdict};

use super::Drive;
use super::bounded_int::bounded_int;
use super::bounded_rat::bounded_rat;
use super::bounded_text::bounded_text;
use std::num::NonZeroU32;

pub(in crate::seals) fn sample_port(
    frame: FrameRef,
    seed: u64,
    i: u32,
    drive: &Drive,
    port: u32,
) -> Value {
    if drive.port == port {
        match frame.id {
            FrameId::Int => return bounded_int(seed, i, drive.bound),
            FrameId::Text => return bounded_text(seed, i, drive.bound),
            FrameId::Rat => return bounded_rat(seed, i, drive.bound),
        }
    }
    match frame.id {
        FrameId::Int => IntFrame::new().generate(seed.wrapping_add(u64::from(i)), 16),
        FrameId::Text => {
            let n = bounded_int(seed, i, NonZeroU32::new(32).unwrap_or(NonZeroU32::MIN));
            let printed = IntFrame::new().print(&n);
            match TextFrame::new().canonicalize(Term::text(&printed)) {
                Verdict::Ok(v) => v,
                Verdict::Refused(_) => TextFrame::new().generate(seed, 4),
            }
        }
        FrameId::Rat => RatFrame::new().generate(seed.wrapping_add(u64::from(i)), 6),
    }
}
