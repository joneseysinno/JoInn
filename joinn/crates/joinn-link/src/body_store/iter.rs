//! Iterate stored bodies by coding hash.

use joinn_dna::{Body, Cell};
use joinn_frame::Hash;
use std::collections::BTreeMap;

use super::BodyStore;

impl BodyStore {
    /// Iterate stored bodies by coding hash.
    pub fn iter(&self) -> impl Iterator<Item = (&Hash, &Body, &BTreeMap<Hash, Cell>)> {
        self.entries.iter().map(|(h, e)| (h, &e.body, &e.cells))
    }
}
