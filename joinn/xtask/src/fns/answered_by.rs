//! The line naming which half of an item's control answered on its mutant.

/// `item <n> control answered by admission | check`.
pub(crate) fn answered_by(n: usize, half: &str) -> String {
    format!("item {n} control answered by {half}")
}
