//! Exit gate 6.

use super::{gate_six_items, run_gate_table};

pub(crate) fn gate_six(phase: &str) -> Result<(u32, u32), String> {
    run_gate_table(phase, gate_six_items())
}
