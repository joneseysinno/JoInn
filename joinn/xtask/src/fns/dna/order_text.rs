//! One order of the fold, as its witness holds it.

use joinn_frame::Verdict;
use joinn_tower::{Dim, directions, level, nodes};

/// Every node's point, then every segment's direction and level.
pub(crate) fn order_text(order: u32) -> Result<String, String> {
    let all = match nodes(Dim(2), order) {
        Verdict::Ok(n) => n,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let dirs = directions(&all);
    let mut out = format!(
        "order {order}: {} nodes, {} segments\n",
        all.len(),
        dirs.len()
    );
    for (d, p) in all.iter().enumerate() {
        out.push_str(&format!("node {d} at ({}, {})\n", p.x, p.y));
    }
    for (i, dir) in dirs.iter().enumerate() {
        let Some(dir) = dir else {
            return Err(format!("order {order}: segment {i} is not one unit step"));
        };
        out.push_str(&format!(
            "segment {i} {} level {}\n",
            dir.name(),
            level(i as u64)
        ));
    }
    Ok(out)
}
