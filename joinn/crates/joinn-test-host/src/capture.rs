//! Captured descriptions, a11y faces, far-side refusals, and the body's intent set.

use joinn_host::{Address, Description};
use joinn_link::LinkRefusal;
use std::collections::BTreeSet;

/// Captured descriptions, a11y faces, far-side refusals, and the body's intent set.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Capture {
    /// One description per fire, in fire order, plus refusals.
    pub descriptions: Vec<Description>,
    /// Link refusals the far side saw. No body reason strings.
    pub far_side: Vec<LinkRefusal>,
    /// The body's derived intent set (host-facing in-ports for a universe run).
    pub intent_set: BTreeSet<Address>,
}
