//! Insert a body under its computed coding hash.

use joinn_dna::{Body, Cell, hash};
use joinn_frame::{Hash, Verdict};
use std::collections::BTreeMap;

use super::{BodyStore, Entry};

impl BodyStore {
    /// Compute the coding hash as the key. Refuses a second body with the same
    /// coding hash and a different regulatory region, naming both sources.
    pub fn insert(
        &mut self,
        body: Body,
        cells: BTreeMap<Hash, Cell>,
        source: &str,
    ) -> Verdict<Hash> {
        let id = hash(&body.coding);
        if let Some(prev) = self.entries.get(&id) {
            if prev.body.regulatory != body.regulatory {
                return crate::refuse(format!(
                    "coding hash has distinct faces: {} and {}",
                    prev.source, source
                ));
            }
            return Verdict::Ok(id);
        }
        self.entries.insert(
            id,
            Entry {
                body,
                cells,
                source: source.to_owned(),
            },
        );
        Verdict::Ok(id)
    }
}
