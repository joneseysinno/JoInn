//! Build a structural refusal for a catalogue miss.

use joinn_frame::{CheckId, Refusal};

pub(super) fn refuse(reason: impl Into<String>) -> Refusal {
    Refusal::structural(CheckId::Other, reason)
}
