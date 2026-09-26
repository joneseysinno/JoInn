//! Canonical H₁ representatives from a spanning forest of the 1-skeleton.

use joinn_frame::Verdict;
use std::collections::{BTreeMap, BTreeSet};

use crate::BlockId;
use crate::chain::Chain;

use super::Complex;
use super::bareiss_rank::bareiss_rank;

/// Cycles kept when they raise the rank of (face boundaries + cycles so far).
pub(super) fn open_cycles(complex: &Complex, blocks: u32) -> Verdict<Vec<Chain>> {
    let vertices: Vec<BlockId> = complex
        .dimension
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(id, _)| *id)
        .collect();
    let edges: Vec<BlockId> = complex
        .dimension
        .iter()
        .filter(|(_, d)| **d == 1)
        .map(|(id, _)| *id)
        .collect();
    let mut edge_at = BTreeMap::new();
    for (i, id) in edges.iter().enumerate() {
        edge_at.insert(*id, i);
    }

    let ends = |id: BlockId| -> Option<(BlockId, BlockId)> {
        let chain = complex.boundaries.get(&id)?;
        if chain.coeff.len() != 2 {
            return None;
        }
        let mut pos = None;
        let mut neg = None;
        for (v, c) in &chain.coeff {
            if *c == 1 && pos.is_none() {
                pos = Some(*v);
            } else if *c == -1 && neg.is_none() {
                neg = Some(*v);
            } else {
                return None;
            }
        }
        Some((neg?, pos?))
    };

    let mut parent: BTreeMap<BlockId, (BlockId, BlockId)> = BTreeMap::new();
    let mut tree: BTreeSet<BlockId> = BTreeSet::new();
    let mut visited: BTreeSet<BlockId> = BTreeSet::new();
    for start in &vertices {
        if !visited.insert(*start) {
            continue;
        }
        let mut queue = vec![*start];
        let mut qh = 0_usize;
        while qh < queue.len() {
            let Some(&v) = queue.get(qh) else {
                break;
            };
            qh += 1;
            for edge in &edges {
                if tree.contains(edge) {
                    continue;
                }
                let Some((tail, head)) = ends(*edge) else {
                    continue;
                };
                let other = if tail == v {
                    head
                } else if head == v {
                    tail
                } else {
                    continue;
                };
                if other == v || visited.contains(&other) {
                    continue;
                }
                visited.insert(other);
                parent.insert(other, (*edge, v));
                tree.insert(*edge);
                queue.push(other);
            }
        }
    }

    let step = |edge: BlockId, from: BlockId, to: BlockId| -> Option<i32> {
        let chain = complex.boundaries.get(&edge)?;
        let at_from = chain.coeff.get(&from).copied().unwrap_or(0);
        let at_to = chain.coeff.get(&to).copied().unwrap_or(0);
        if at_to == 1 && at_from == -1 {
            Some(1)
        } else if at_to == -1 && at_from == 1 {
            Some(-1)
        } else {
            None
        }
    };

    let mut fundamentals: Vec<BTreeMap<BlockId, i32>> = Vec::new();
    for edge in &edges {
        if tree.contains(edge) {
            continue;
        }
        let Some((tail, head)) = ends(*edge) else {
            continue;
        };
        if tail == head {
            continue;
        }
        let mut ancestors = BTreeSet::new();
        let mut walk = tail;
        ancestors.insert(walk);
        while let Some((_, up)) = parent.get(&walk) {
            walk = *up;
            ancestors.insert(walk);
        }
        let mut head_steps: Vec<(BlockId, i32)> = Vec::new();
        let mut lca = head;
        while !ancestors.contains(&lca) {
            let Some(&(pedge, up)) = parent.get(&lca) else {
                break;
            };
            let Some(coeff) = step(pedge, lca, up) else {
                break;
            };
            head_steps.push((pedge, coeff));
            lca = up;
        }
        if !ancestors.contains(&lca) {
            continue;
        }
        let mut tail_steps: Vec<(BlockId, i32)> = Vec::new();
        let mut cur = tail;
        let mut broke = false;
        while cur != lca {
            let Some(&(pedge, up)) = parent.get(&cur) else {
                broke = true;
                break;
            };
            let Some(coeff) = step(pedge, cur, up) else {
                broke = true;
                break;
            };
            tail_steps.push((pedge, coeff));
            cur = up;
        }
        if broke {
            continue;
        }
        let mut coeff: BTreeMap<BlockId, i32> = BTreeMap::new();
        let mut fit = true;
        let mut add = |id: BlockId, c: i32| {
            if !fit {
                return;
            }
            let sum = match coeff.get(&id).copied().unwrap_or(0).checked_add(c) {
                Some(s) => s,
                None => {
                    fit = false;
                    return;
                }
            };
            if sum == 0 {
                coeff.remove(&id);
            } else {
                coeff.insert(id, sum);
            }
        };
        add(*edge, 1);
        for (id, c) in &head_steps {
            add(*id, *c);
        }
        for (id, c) in &tail_steps {
            let Some(neg) = (*c).checked_neg() else {
                fit = false;
                break;
            };
            add(*id, neg);
        }
        if !fit || coeff.is_empty() {
            continue;
        }
        if let Some(c) = coeff.values().next().copied() {
            if c < 0 {
                let mut flipped: BTreeMap<BlockId, i32> = BTreeMap::new();
                for (id, c) in &coeff {
                    let Some(neg) = (*c).checked_neg() else {
                        fit = false;
                        break;
                    };
                    flipped.insert(*id, neg);
                }
                if fit {
                    coeff = flipped;
                }
            }
        }
        if fit {
            fundamentals.push(coeff);
        }
    }

    let column = |coeff: &BTreeMap<BlockId, i32>| -> Vec<i128> {
        let mut col = vec![0_i128; edges.len()];
        for (id, c) in coeff {
            if let Some(&i) = edge_at.get(id) {
                if let Some(slot) = col.get_mut(i) {
                    *slot = i128::from(*c);
                }
            }
        }
        col
    };

    let as_matrix = |cols: &[Vec<i128>]| -> Vec<Vec<i128>> {
        if cols.is_empty() {
            return Vec::new();
        }
        let width = cols.len();
        let height = cols.first().map(Vec::len).unwrap_or(0);
        let mut matrix = vec![vec![0_i128; width]; height];
        for (c, col) in cols.iter().enumerate() {
            for (r, value) in col.iter().enumerate() {
                if let Some(slot) = matrix.get_mut(r).and_then(|row| row.get_mut(c)) {
                    *slot = *value;
                }
            }
        }
        matrix
    };

    let mut span: Vec<Vec<i128>> = Vec::new();
    for (id, dim) in &complex.dimension {
        if *dim != 2 {
            continue;
        }
        let Some(boundary) = complex.boundaries.get(id) else {
            continue;
        };
        let mut coeff = BTreeMap::new();
        for (face, c) in &boundary.coeff {
            if edge_at.contains_key(face) && *c != 0 {
                coeff.insert(*face, *c);
            }
        }
        span.push(column(&coeff));
    }

    let mut open = Vec::new();
    for coeff in &fundamentals {
        let before = match bareiss_rank(as_matrix(&span), blocks) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let mut trial = span.clone();
        trial.push(column(coeff));
        let after = match bareiss_rank(as_matrix(&trial), blocks) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        if after > before {
            span.push(column(coeff));
            open.push(Chain::from_coeffs(coeff.iter().map(|(id, c)| (*id, *c))));
        }
    }
    Verdict::Ok(open)
}
