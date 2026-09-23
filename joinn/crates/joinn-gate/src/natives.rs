//! Native allele registry. The mechanism, not the alleles.

mod get_arc;

use crate::oracle::Oracle;
use joinn_dna::NativeId;
use std::collections::BTreeMap;
use std::sync::Arc;

/// NativeId → oracle. Alleles themselves live in `joinn-prim`.
#[derive(Clone, Default)]
pub struct NativeRegistry {
    pub(in crate::natives) map: BTreeMap<NativeId, Arc<dyn Oracle>>,
}

impl NativeRegistry {
    /// Empty register.
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    /// Insert under a registered name.
    pub fn insert(&mut self, name: impl Into<String>, oracle: Arc<dyn Oracle>) {
        self.map.insert(NativeId(name.into()), oracle);
    }

    /// Resolve.
    pub fn get(&self, id: &NativeId) -> Option<&dyn Oracle> {
        self.map.get(id).map(|a| a.as_ref())
    }
}
