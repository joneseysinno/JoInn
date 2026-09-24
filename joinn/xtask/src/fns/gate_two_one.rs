//! Exit gate 2.1 (legacy): checks only.

use super::{gate_two_one_items, run_legacy_table};

pub(crate) fn gate_two_one(phase: &str) -> Result<(u32, u32), String> {
    run_legacy_table(phase, gate_two_one_items())
}
