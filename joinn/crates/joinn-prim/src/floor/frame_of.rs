//! Frame instance for a value's frame id.

use joinn_frame::{Frame, FrameId, IntFrame, RatFrame, TextFrame, Value};

pub(in crate::floor) fn frame_of(v: &Value) -> Box<dyn Frame> {
    match v.frame().id {
        FrameId::Int => Box::new(IntFrame::new()),
        FrameId::Text => Box::new(TextFrame::new()),
        FrameId::Rat => Box::new(RatFrame::new()),
    }
}
