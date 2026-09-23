//! Exit gate 5: artifact-opposed items.

use super::{gate_five_items, run_gate_table};

pub(crate) fn gate_five() -> Result<(u32, u32), String> {
    run_gate_table(&format!("phase {}", 5), gate_five_items())
}
