//! Live grants that crossed a system boundary on an ordered link.

use std::collections::BTreeSet;

/// Live grants that crossed a system boundary on an ordered link.
#[derive(Clone, Debug, Default)]
pub struct LinkRuntime {
    /// (capability name, receiving body alias) currently granted.
    pub(crate) held: BTreeSet<(String, String)>,
}
