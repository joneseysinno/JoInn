//! Zoom out to a floor.

use super::level;

/// The items on segments of level ≥ `floor`, in order; item i sits on
/// segment i.
pub fn level_filter<T: Clone>(items: &[T], floor: u32) -> Vec<T> {
    items
        .iter()
        .enumerate()
        .filter(|(i, _)| level(*i as u64) >= floor)
        .map(|(_, item)| item.clone())
        .collect()
}
