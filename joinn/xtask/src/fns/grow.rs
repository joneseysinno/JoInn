//! `cargo xtask grow`: counting and adding grow, every size true (plan 7.4 §3).

mod corpus_system;
mod grow_measure;
mod plants;
mod run;
mod step_fault;

pub(crate) use corpus_system::corpus_system;
pub(crate) use grow_measure::grow_measure;
pub(crate) use run::grow;
pub(crate) use step_fault::step_fault;

use joinn_dna::{Accept, Contact, System};
use joinn_frame::Hash;
use std::collections::BTreeMap;

/// The systems `grow` reports, in order: `corpus/phase74/<name>.system`.
pub(crate) const GROW_SYSTEMS: [&str; 2] = ["counting", "adding"];

/// Seven inputs each body accepts, for `regrow` and `pick`: counting `1`s,
/// adding seven others.
pub(crate) fn seven_inputs(accepts: Accept) -> [i64; 7] {
    match accepts {
        Accept::One => [1; 7],
        Accept::Any => [2, 3, 4, -2, 0, 7, 12],
    }
}

/// A corpus system with the growing contacts it binds and its `hashes.txt` row.
pub(crate) struct CorpusSystem {
    pub(crate) name: String,
    pub(crate) system: System,
    pub(crate) contacts: BTreeMap<Hash, Contact>,
    pub(crate) accepts: Accept,
    pub(crate) waiting: u32,
    pub(crate) golden: String,
}
