//! Exit gate 2.2 (legacy): checks only.

use super::{gate_two_two_items, run_legacy_table};

pub(crate) fn gate_two_two() -> Result<(u32, u32), String> {
    run_legacy_table("phase 2.2", gate_two_two_items())
}
