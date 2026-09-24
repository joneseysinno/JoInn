//! Exit gate 1 (legacy): checks only.

use super::{gate_one_items, run_legacy_table};

pub(crate) fn gate_one() -> Result<(u32, u32), String> {
    run_legacy_table("phase 1", gate_one_items())
}
