//! `NativeRegistry::get_arc`.

use super::NativeRegistry;
use crate::oracle::Oracle;
use joinn_dna::NativeId;
use std::sync::Arc;

impl NativeRegistry {
    /// Resolve as an `Arc` for `CellAt` registration.
    pub fn get_arc(&self, id: &NativeId) -> Option<Arc<dyn Oracle>> {
        self.map.get(id).cloned()
    }
}
