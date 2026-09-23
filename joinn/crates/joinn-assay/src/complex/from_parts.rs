//! Build a complex from dimension and boundary tables.

use std::collections::BTreeMap;

use crate::chain::Chain;
use crate::BlockId;

use super::Complex;

impl Complex {
    /// Build a complex from dimension and boundary tables.
    pub fn from_parts(
        dimension: BTreeMap<BlockId, u32>,
        boundaries: BTreeMap<BlockId, Chain>,
    ) -> Self {
        Self {
            dimension,
            boundaries,
        }
    }
}
