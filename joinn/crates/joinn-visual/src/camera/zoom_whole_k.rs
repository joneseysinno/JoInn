//! `k` as a whole number, when it is one.

use super::Zoom;

impl Zoom {
    /// `Some(k)` when `k` is a whole number of pixels per layout unit.
    pub fn whole_k(self) -> Option<i64> {
        match self.k() {
            (num, 1) => Some(num),
            _ => None,
        }
    }
}
