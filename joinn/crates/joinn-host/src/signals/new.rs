//! Build a declared signal set.

use std::collections::BTreeSet;

use super::Signals;

impl Signals {
    /// Build a declared signal set.
    pub fn new(names: BTreeSet<String>) -> Self {
        Self { names }
    }
}
