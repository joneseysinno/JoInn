//! Captured descriptions, a11y faces, and the body's intent set.

use joinn_host::{Address, Description};
use std::collections::BTreeSet;

/// Captured descriptions, a11y faces, and the body's intent set.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Capture {
    /// One description per fire, in fire order, plus refusals.
    pub descriptions: Vec<Description>,
    /// The body's derived intent set.
    pub intent_set: BTreeSet<Address>,
}
