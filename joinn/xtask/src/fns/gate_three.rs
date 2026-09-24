//! Exit gate 3 (legacy): checks only.

use super::{gate_three_items, run_legacy_table};

pub(crate) fn gate_three() -> Result<(u32, u32), String> {
    run_legacy_table("phase 3", gate_three_items())
}
