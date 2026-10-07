//! `cargo xtask grow`: counting and adding grow, every size true (plan 7.4 §3).

mod corpus_system;
mod plants;
mod run;
mod step_fault;

pub(crate) use corpus_system::corpus_system;
pub(crate) use run::grow;
pub(crate) use step_fault::step_fault;

use joinn_dna::{Accept, Contact, System};
use joinn_frame::Hash;
use std::collections::BTreeMap;

/// The systems `grow` reports, in order: `corpus/phase74/<name>.system`.
pub(crate) const GROW_SYSTEMS: [&str; 2] = ["counting", "adding"];

/// A corpus system with the growing contacts it binds and its `hashes.txt` row.
pub(crate) struct CorpusSystem {
    pub(crate) name: String,
    pub(crate) system: System,
    pub(crate) contacts: BTreeMap<Hash, Contact>,
    pub(crate) accepts: Accept,
    pub(crate) waiting: u32,
    pub(crate) golden: String,
}
