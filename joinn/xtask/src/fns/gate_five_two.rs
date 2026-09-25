//! Exit gate 5.2.

use super::{gate_five_two_items, run_gate_table};

pub(crate) fn gate_five_two(phase: &str) -> Result<(u32, u32), String> {
    run_gate_table(phase, gate_five_two_items())
}
