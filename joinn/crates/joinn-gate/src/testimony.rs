//! Accumulated testimony. Never hashed. Never identity.

mod append;
mod memory;
mod records;

use joinn_frame::Hash;
use std::collections::BTreeMap;

/// Append-only store, keyed by cell hash. Memory only.
#[derive(Clone, Debug, Default)]
pub struct TestimonyStore {
    memory: BTreeMap<Hash, Vec<String>>,
}
