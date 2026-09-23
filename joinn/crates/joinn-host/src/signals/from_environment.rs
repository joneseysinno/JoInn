//! The environment body names the signals a host emits.

use joinn_dna::Body;
use std::collections::BTreeSet;

use super::Signals;

/// Each genome instance of the environment body is a signal this host emits.
pub fn signals_from_environment(body: &Body) -> Signals {
    let mut names = BTreeSet::new();
    for entry in &body.coding.genome {
        for instance in &entry.instances {
            names.insert(instance.clone());
        }
    }
    Signals::new(names)
}
