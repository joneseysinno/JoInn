//! Exit gate 3: artifact-opposed items.

use super::{gate_three_items, run_gate_table};

/// Exit gate 3: artifact-opposed items.
pub(crate) fn gate_three() -> Result<(u32, u32), String> {
    run_gate_table("phase 3", gate_three_items())
}
