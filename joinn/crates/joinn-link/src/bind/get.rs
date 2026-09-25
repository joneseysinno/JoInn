//! Lookup one bound alias.

use super::{BodyWithCells, Bound};

impl Bound {
    /// Lookup one bound alias.
    pub fn get(&self, alias: &str) -> Option<&BodyWithCells> {
        self.by_alias.get(alias)
    }
}
