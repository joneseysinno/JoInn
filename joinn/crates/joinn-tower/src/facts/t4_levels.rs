//! T4: a segment's level is the block sizes it bridges.

use std::collections::BTreeMap;

use joinn_frame::Verdict;

use crate::level::level;
use crate::refuse::refuse;

/// For every segment i, the trailing 3s of i in base 4 equal the number of
/// block sizes 4ᵏ (k = 1 … n) whose block changes between node i and i + 1.
pub(crate) fn t4_levels(order: u32) -> Verdict<String> {
    let segments = (1u64 << (2 * order)) - 1;
    let mut counts: BTreeMap<u32, u64> = BTreeMap::new();
    for i in 0..segments {
        let bridged = (1..=order)
            .filter(|k| (i >> (2 * k)) != ((i + 1) >> (2 * k)))
            .count() as u32;
        if level(i) != bridged {
            return refuse(format!(
                "tower: order {order} segment {i} has level {} but bridges {bridged} block sizes; acceptance is the trailing 3s of i in base 4",
                level(i)
            ));
        }
        *counts.entry(bridged).or_default() += 1;
    }
    let parts: Vec<String> = counts.iter().map(|(l, c)| format!("{l} ×{c}")).collect();
    Verdict::Ok(format!("levels {}", parts.join(", ")))
}
