//! Peek the current value in a slot without consuming.

use joinn_frame::Value;

use crate::slot::Slot;

pub(crate) fn slot_value(slot: Option<&Slot>) -> Option<Value> {
    match slot {
        Some(Slot::Filled(v)) => Some(v.clone()),
        Some(Slot::Queue(q)) if !q.is_empty() => Some(q[0].clone()),
        _ => None,
    }
}
