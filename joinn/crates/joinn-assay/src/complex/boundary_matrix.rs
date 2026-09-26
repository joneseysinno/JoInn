//! Boundary matrix of one dimension, rows and columns in block-id order.

use std::collections::BTreeMap;

use crate::BlockId;

use super::Complex;

/// Rows are `(dim − 1)`-blocks, columns are `dim`-blocks, both in id order.
pub(super) fn boundary_matrix(complex: &Complex, dim: u32) -> Vec<Vec<i128>> {
    if dim == 0 {
        return Vec::new();
    }
    let rows: Vec<BlockId> = complex
        .dimension
        .iter()
        .filter(|(_, d)| **d + 1 == dim)
        .map(|(id, _)| *id)
        .collect();
    let cols: Vec<BlockId> = complex
        .dimension
        .iter()
        .filter(|(_, d)| **d == dim)
        .map(|(id, _)| *id)
        .collect();
    if cols.is_empty() {
        return Vec::new();
    }
    let mut index = BTreeMap::new();
    for (i, id) in rows.iter().enumerate() {
        index.insert(*id, i);
    }
    let mut matrix = vec![vec![0_i128; cols.len()]; rows.len()];
    for (c, col_id) in cols.iter().enumerate() {
        let Some(chain) = complex.boundaries.get(col_id) else {
            continue;
        };
        for (face, coeff) in &chain.coeff {
            let Some(&r) = index.get(face) else {
                continue;
            };
            if let Some(slot) = matrix.get_mut(r).and_then(|row| row.get_mut(c)) {
                *slot = i128::from(*coeff);
            }
        }
    }
    matrix
}
