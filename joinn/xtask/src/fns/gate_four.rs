//! Exit gate 4. Each item's subject must also hold still under the invariance harness.

use super::{assay_invariance, gate_four_items, run_gate_table};

pub(crate) fn gate_four(phase: &str) -> Result<(u32, u32), String> {
    let lines = assay_invariance()?;
    for (i, item) in gate_four_items().iter().enumerate() {
        let rel = item
            .control_artifact
            .strip_prefix("corpus/")
            .unwrap_or(item.control_artifact);
        let prefix = format!("{rel}: ");
        let Some(line) = lines.lines().find(|l| l.starts_with(&prefix)) else {
            return Err(format!(
                "gate item {} ({}): the invariance harness did not measure {rel}; acceptance is a line for that subject",
                i + 1,
                item.name
            ));
        };
        println!("invariance {line}");
    }
    run_gate_table(phase, gate_four_items())
}
