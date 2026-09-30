//! Exit gate 7.

use super::{gate_seven_items, run_gate_table};

pub(crate) fn gate_seven(phase: &str) -> Result<(u32, u32), String> {
    run_gate_table(phase, gate_seven_items())
}
