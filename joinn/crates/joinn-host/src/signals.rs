//! Environment signals a host emits. A body may only read declared ones.

mod check_signals;
mod from_environment;
mod new;

pub use check_signals::check_signals;
pub use from_environment::signals_from_environment;

use std::collections::BTreeSet;

/// The set of environment out-ports this host emits.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Signals {
    /// Signal names, ordered.
    pub names: BTreeSet<String>,
}

impl Signals {
    /// True when this host emits `name`.
    pub fn emits(&self, name: &str) -> bool {
        self.names.contains(name)
    }
}
