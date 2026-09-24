//! Exit gate 2.2 (legacy): checks only.

use super::{gate_two_two_items, run_legacy_table};

pub(crate) fn gate_two_two(phase: &str) -> Result<(u32, u32), String> {
    run_legacy_table(phase, gate_two_two_items())
}
