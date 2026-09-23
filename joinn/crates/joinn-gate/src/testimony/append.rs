//! Append one canonical-text record. Does not touch a cell hash.

use joinn_frame::Hash;

use super::TestimonyStore;

impl TestimonyStore {
    /// Append one canonical-text record. Does not touch a cell hash.
    pub fn append(&mut self, cell: Hash, record: &str) {
        self.memory.entry(cell).or_default().push(record.to_owned());
    }
}
