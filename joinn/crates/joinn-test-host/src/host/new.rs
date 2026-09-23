//! A host that declares no environment signals.

use joinn_dna::{Body, Cell};
use joinn_frame::Hash;
use joinn_host::Signals;
use std::collections::{BTreeMap, BTreeSet};

use super::TestHost;

impl TestHost {
    /// A host that declares no environment signals.
    pub fn new(body: Body, cells: BTreeMap<Hash, Cell>) -> Self {
        Self {
            captures: Vec::new(),
            body,
            cells,
            signals: Signals::new(BTreeSet::new()),
        }
    }
}
