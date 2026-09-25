//! Look up a body by coding hash.

use joinn_dna::{Body, Cell};
use joinn_frame::Hash;
use std::collections::BTreeMap;

use super::BodyStore;

impl BodyStore {
    pub(crate) fn get(&self, id: &Hash) -> Option<(&Body, &BTreeMap<Hash, Cell>)> {
        self.entries.get(id).map(|e| (&e.body, &e.cells))
    }
}
