//! Opposed gate table runner.

use super::{Artifact, GateItem};

/// Run an opposed table. Fails naming the item if a control passes on the real bytes.
pub fn run_opposed(
    items: &[GateItem],
    artifacts: &[&[u8]],
) -> Result<Vec<(usize, &'static str, bool)>, String> {
    if items.len() != artifacts.len() {
        return Err(format!(
            "gate table has {} items and {} artifacts; acceptance is one artifact per item",
            items.len(),
            artifacts.len()
        ));
    }
    let mut rows = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let n = i + 1;
        let art = Artifact {
            path: item.control_artifact,
            bytes: artifacts[i],
        };
        if (item.control)(&art) {
            return Err(format!("gate item {n} ({}) control passed", item.name));
        }
        rows.push((n, item.name, (item.check)()));
    }
    Ok(rows)
}
