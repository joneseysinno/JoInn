//! Captured descriptions, a11y faces, far-side refusals, and each alias's intent set.

use joinn_host::{Address, Description};
use joinn_link::LinkRefusal;
use std::collections::{BTreeMap, BTreeSet};

/// Captured descriptions, a11y faces, far-side refusals, and each alias's intent set.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Capture {
    /// One description per fire, in fire order, plus refusals.
    pub descriptions: Vec<Description>,
    /// Link refusals the far side saw. No body reason strings.
    pub far_side: Vec<LinkRefusal>,
    /// One host-facing intent set per bound alias.
    pub intent_set: BTreeMap<String, BTreeSet<Address>>,
}
