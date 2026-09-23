//! Port slot representation. Mutation only through `apply_slot` (V35).

mod apply;
mod ready;
mod value;

use joinn_frame::Value;

pub(crate) use apply::apply_slot;
pub(crate) use ready::slot_ready;
pub(crate) use value::slot_value;

pub(crate) enum Slot {
    Empty,
    Filled(Value),
    Queue(Vec<Value>),
}

pub(crate) enum SlotWrite {
    Fill(Value),
    Consume,
}
