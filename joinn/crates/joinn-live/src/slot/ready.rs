//! Whether a slot holds a value ready to fire.

use crate::slot::Slot;

pub(crate) fn slot_ready(slot: Option<&Slot>) -> bool {
    match slot {
        Some(Slot::Filled(_)) => true,
        Some(Slot::Queue(q)) if !q.is_empty() => true,
        _ => false,
    }
}
