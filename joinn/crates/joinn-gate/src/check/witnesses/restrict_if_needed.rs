//! Cross-frame restrict helper for witness replay.

use joinn_frame::{FrameRef, FrameRegistry, Verdict};

pub(in crate::check::witnesses) fn restrict_if_needed(
    v: &joinn_frame::Value,
    target: &FrameRef,
    frames: &FrameRegistry,
) -> Verdict<joinn_frame::Value> {
    if v.frame() == target {
        return Verdict::Ok(v.clone());
    }
    let Some(src) = frames.get(v.frame()) else {
        return Verdict::Ok(v.clone());
    };
    src.restrict(target, v)
}
