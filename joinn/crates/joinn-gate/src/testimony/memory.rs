//! In-memory only.

use super::TestimonyStore;
use std::collections::BTreeMap;

impl TestimonyStore {
    /// In-memory only.
    pub fn memory() -> Self {
        Self {
            memory: BTreeMap::new(),
        }
    }
}
