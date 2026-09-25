//! Blocks and the boundary chain stored with each one.

mod assemble;
mod boundary;
mod from_parts;

use std::collections::BTreeMap;

use crate::BlockId;
use crate::chain::Chain;

/// Blocks by dimension, each with the chain that is its boundary.
pub struct Complex {
    /// Dimension of each block that belongs to this complex.
    pub(crate) dimension: BTreeMap<BlockId, u32>,
    /// Boundary of each block, one dimension down. Empty for a 0-block.
    pub(crate) boundaries: BTreeMap<BlockId, Chain>,
}
