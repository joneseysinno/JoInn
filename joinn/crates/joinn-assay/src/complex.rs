//! Blocks and the boundary chain stored with each one.

mod assemble;
mod bareiss_rank;
mod boundary;
mod boundary_matrix;
mod from_parts;
mod homology;
mod open_cycles;

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

/// Exact ranks over ℚ, with one canonical generator per H₁ class.
pub struct Homology {
    /// Rank of H₀.
    pub b0: u32,
    /// Rank of H₁.
    pub b1: u32,
    /// Rank of H₂.
    pub b2: u32,
    /// Canonical H₁ representatives, in fundamental-cycle order.
    pub open: Vec<Chain>,
}
