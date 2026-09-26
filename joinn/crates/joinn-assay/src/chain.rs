//! A signed formal sum of blocks of one dimension. Zero coefficients are dropped.

use std::collections::BTreeMap;

use crate::BlockId;

/// A signed formal sum of blocks. Equal chains are equal as maps.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Chain {
    pub(crate) coeff: BTreeMap<BlockId, i32>,
}

impl Chain {
    /// Build a chain. A zero coefficient is not stored.
    pub fn from_coeffs(pairs: impl IntoIterator<Item = (BlockId, i32)>) -> Self {
        let mut coeff = BTreeMap::new();
        for (id, c) in pairs {
            if c == 0 {
                continue;
            }
            let sum = coeff.get(&id).copied().unwrap_or(0_i32).saturating_add(c);
            if sum == 0 {
                coeff.remove(&id);
            } else {
                coeff.insert(id, sum);
            }
        }
        Self { coeff }
    }
}
