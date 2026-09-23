//! Construct a CLI host from a declared signal set.

use joinn_dna::{Body, Cell};
use joinn_frame::Hash;
use joinn_host::Signals;
use std::collections::BTreeMap;

use super::CliHost;

impl CliHost {
    /// Construct a CLI host from a declared signal set.
    pub(crate) fn new(body: Body, cells: BTreeMap<Hash, Cell>, signals: Signals) -> Self {
        Self {
            out: String::new(),
            body,
            cells,
            pending: None,
            signals,
        }
    }
}
