//! Exit gate 5.1.

use super::{gate_five_one_items, run_gate_table};

pub(crate) fn gate_five_one(phase: &str) -> Result<(u32, u32), String> {
    run_gate_table(phase, gate_five_one_items())
}
