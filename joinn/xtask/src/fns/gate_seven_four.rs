//! Gate 7.4 runner.

use super::{gate_seven_four_items, run_gate_table};

pub(crate) fn gate_seven_four(phase: &str) -> Result<(u32, u32), String> {
    run_gate_table(phase, gate_seven_four_items())
}
