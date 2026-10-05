//! Gate 7.2 runner.

use super::{gate_seven_two_items, run_gate_table};

pub(crate) fn gate_seven_two(phase: &str) -> Result<(u32, u32), String> {
    run_gate_table(phase, gate_seven_two_items())
}
