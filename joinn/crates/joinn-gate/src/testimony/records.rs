//! All records for a cell, memory first.

use joinn_frame::Hash;

use super::TestimonyStore;

impl TestimonyStore {
    /// All records for a cell, memory first.
    pub fn records(&self, cell: Hash) -> Vec<String> {
        self.memory.get(&cell).cloned().unwrap_or_default()
    }
}
