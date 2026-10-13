//! Gate 8 runner.

use super::{gate_eight_items, run_gate_table};

pub(crate) fn gate_eight(phase: &str) -> Result<(u32, u32), String> {
    run_gate_table(phase, gate_eight_items())
}
