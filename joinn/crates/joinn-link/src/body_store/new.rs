//! Construct an empty body store.

use super::BodyStore;

impl BodyStore {
    /// Empty store.
    pub fn new() -> Self {
        Self::default()
    }
}
