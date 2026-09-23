//! `values_eq`.

use joinn_frame::{FrameRef, FrameRegistry, Value, Verdict};

pub(in crate::eval) fn values_eq(a: &Value, b: &Value, frames: &FrameRegistry) -> bool {
    if a.frame() == b.frame() {
        return frames.get(a.frame()).is_some_and(|f| f.eq(a, b));
    }
    // Try embedding the narrower into the wider.
    if a.frame().id == joinn_frame::FrameId::Int && b.frame().id == joinn_frame::FrameId::Rat {
        if let Some(rat) = frames.get(&FrameRef::rat()) {
            if let Verdict::Ok(ae) = rat.embed(a.frame(), a) {
                return rat.eq(&ae, b);
            }
        }
    }
    if b.frame().id == joinn_frame::FrameId::Int && a.frame().id == joinn_frame::FrameId::Rat {
        if let Some(rat) = frames.get(&FrameRef::rat()) {
            if let Verdict::Ok(be) = rat.embed(b.frame(), b) {
                return rat.eq(a, &be);
            }
        }
    }
    false
}
