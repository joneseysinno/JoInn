//! Cells from any stored body.

use joinn_dna::Cell;
use joinn_frame::Hash;
use std::collections::BTreeMap;

use super::BodyStore;

impl BodyStore {
    /// Cells from any stored body (phase loaders share one cell map).
    pub fn any_cells(&self) -> Option<&BTreeMap<Hash, Cell>> {
        self.entries.values().next().map(|e| &e.cells)
    }
}
