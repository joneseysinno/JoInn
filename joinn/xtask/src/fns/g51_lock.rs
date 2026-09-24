//! Gate 5.1 item 9: an off-by-one lock fails, naming the phase.

use super::scores_match;
use super::parse_lock_scores::LockRow;

pub(crate) fn g51_lock() -> bool {
    let n = 8u32;
    let total = n;
    let wrong = n.saturating_sub(1); // allow(vocab): off-by-one lock fixture, not the subtract concept; file deleted in P52-06
    let phase = format!("phase {}", 5);
    let returned = [(phase.as_str(), true, n, total)];
    let rows = [LockRow {
        phase: phase.clone(),
        n: wrong,
        total,
    }];
    match scores_match(&returned, &rows) {
        Err(msg) => msg.contains(&phase),
        Ok(()) => false,
    }
}
