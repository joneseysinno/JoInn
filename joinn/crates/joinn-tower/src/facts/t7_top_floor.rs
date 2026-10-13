//! T7: the top floor.

use joinn_frame::Verdict;

use crate::level::level;
use crate::refuse::refuse;

/// Exactly three segments have level n − 1, at 4ⁿ⁻¹·k − 1 for k = 1, 2, 3;
/// the middle one is the centre segment.
pub(crate) fn t7_top_floor(order: u32) -> Verdict<String> {
    let top = order.saturating_sub(1);
    let segments = (1u64 << (2 * order)) - 1;
    let found: Vec<u64> = (0..segments).filter(|i| level(*i) == top).collect();
    let quarter = 1u64 << (2 * top);
    let want: Vec<u64> = (1..=3).map(|k| quarter * k - 1).collect();
    let centre = segments.saturating_sub(1) / 2;
    if found != want || want.get(1) != Some(&centre) {
        return refuse(format!(
            "tower: order {order} has level-{top} segments {found:?}; acceptance is {want:?} with the middle at the centre, {centre}"
        ));
    }
    let list: Vec<String> = found.iter().map(u64::to_string).collect();
    Verdict::Ok(format!(
        "top floor (level {top}): segments {}; the middle, {centre}, is the centre",
        list.join(", ")
    ))
}
