//! Exit gate 5: artifact-opposed items.

use super::{gate_five_items, run_gate_table};

pub(crate) fn gate_five(phase: &str) -> Result<(u32, u32), String> {
    run_gate_table(phase, gate_five_items())
}
