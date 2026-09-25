//! Iterate every bound alias.

use super::{BodyWithCells, Bound};

impl Bound {
    /// Every alias in binding order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &BodyWithCells)> {
        self.by_alias.iter().map(|(k, v)| (k.as_str(), v))
    }
}
