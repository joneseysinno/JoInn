//! Exact Betti numbers over ℚ, or a refusal when elimination does not fit.

use joinn_frame::{CheckId, Refusal, Verdict};

use super::bareiss_rank::bareiss_rank;
use super::boundary_matrix::boundary_matrix;
use super::open_cycles::open_cycles;
use super::{Complex, Homology};

impl Complex {
    /// Ranks b₀, b₁, b₂ and the canonical open cycles. Euler must hold.
    pub fn homology(&self) -> Verdict<Homology> {
        let blocks = u32::try_from(self.dimension.len()).unwrap_or(u32::MAX);
        let count = |dim: u32| -> u32 {
            let n = self.dimension.values().filter(|d| **d == dim).count();
            u32::try_from(n).unwrap_or(u32::MAX)
        };
        let n0 = count(0);
        let n1 = count(1);
        let n2 = count(2);
        let rank = |dim: u32| bareiss_rank(boundary_matrix(self, dim), blocks);
        let r1 = match rank(1) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let r2 = match rank(2) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let r3 = match rank(3) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let refuse_rank = || {
            Verdict::Refused(Refusal::structural(
                CheckId::Other,
                format!(
                    "rank exceeded the blocks of a complex with {blocks} blocks; acceptance is a rank at most the block count"
                ),
            ))
        };
        let Some(b0) = n0.checked_sub(r1) else {
            return refuse_rank();
        };
        let Some(b1) = n1.checked_sub(r1).and_then(|n| n.checked_sub(r2)) else {
            return refuse_rank();
        };
        let Some(b2) = n2.checked_sub(r2).and_then(|n| n.checked_sub(r3)) else {
            return refuse_rank();
        };
        let left = i64::from(n0) - i64::from(n1) + i64::from(n2);
        let right = i64::from(b0) - i64::from(b1) + i64::from(b2);
        if left != right {
            return Verdict::Refused(Refusal::structural(
                CheckId::Other,
                format!(
                    "euler {n0} − {n1} + {n2} = {left} ≠ {right} = {b0} − {b1} + {b2}; acceptance is equality"
                ),
            ));
        }
        let open = match open_cycles(self, blocks) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        Verdict::Ok(Homology { b0, b1, b2, open })
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use std::collections::{BTreeMap, BTreeSet};

    use crate::{BlockId, Chain, Complex};

    fn complex(blocks: &[(u32, u32, Chain)]) -> Complex {
        let mut dimension = BTreeMap::new();
        let mut boundaries = BTreeMap::new();
        for (id, dim, chain) in blocks {
            dimension.insert(BlockId(*id), *dim);
            boundaries.insert(BlockId(*id), chain.clone());
        }
        Complex {
            dimension,
            boundaries,
        }
    }

    fn assert_betti(built: &Complex, expect: (u32, u32, u32)) -> crate::Homology {
        match built.assemble() {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => panic!("assemble: {}", r.reason),
        }
        match built.homology() {
            Verdict::Ok(h) => {
                assert_eq!((h.b0, h.b1, h.b2), expect);
                let n = |dim: u32| -> i64 {
                    i64::try_from(built.dimension.values().filter(|d| **d == dim).count())
                        .unwrap_or(0)
                };
                let left = n(0) - n(1) + n(2);
                let right = i64::from(h.b0) - i64::from(h.b1) + i64::from(h.b2);
                assert_eq!(left, right, "euler");
                h
            }
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    fn edge(head: u32, tail: u32) -> Chain {
        Chain::from_coeffs([(BlockId(head), 1), (BlockId(tail), -1)])
    }

    #[test]
    fn a_filled_triangle_is_one_zero_zero() {
        let h = assert_betti(
            &complex(&[
                (0, 0, Chain::from_coeffs([])),
                (1, 0, Chain::from_coeffs([])),
                (2, 0, Chain::from_coeffs([])),
                (3, 1, edge(1, 0)),
                (4, 1, edge(2, 1)),
                (5, 1, edge(0, 2)),
                (
                    6,
                    2,
                    Chain::from_coeffs([(BlockId(3), 1), (BlockId(4), 1), (BlockId(5), 1)]),
                ),
            ]),
            (1, 0, 0),
        );
        assert!(h.open.is_empty());
    }

    #[test]
    fn a_hollow_triangle_names_its_open_cycle() {
        let h = assert_betti(
            &complex(&[
                (0, 0, Chain::from_coeffs([])),
                (1, 0, Chain::from_coeffs([])),
                (2, 0, Chain::from_coeffs([])),
                (3, 1, edge(1, 0)),
                (4, 1, edge(2, 1)),
                (5, 1, edge(0, 2)),
            ]),
            (1, 1, 0),
        );
        assert_eq!(
            h.open,
            vec![Chain::from_coeffs([
                (BlockId(3), 1),
                (BlockId(4), 1),
                (BlockId(5), 1)
            ])]
        );
    }

    #[test]
    fn a_tetrahedron_surface_is_one_zero_one() {
        // Edges 4..9 are 0→1, 0→2, 0→3, 1→2, 1→3, 2→3.
        // Faces are oriented so each edge meets two faces with opposite signs.
        let h = assert_betti(
            &complex(&[
                (0, 0, Chain::from_coeffs([])),
                (1, 0, Chain::from_coeffs([])),
                (2, 0, Chain::from_coeffs([])),
                (3, 0, Chain::from_coeffs([])),
                (4, 1, edge(1, 0)),
                (5, 1, edge(2, 0)),
                (6, 1, edge(3, 0)),
                (7, 1, edge(2, 1)),
                (8, 1, edge(3, 1)),
                (9, 1, edge(3, 2)),
                (
                    10,
                    2,
                    Chain::from_coeffs([(BlockId(4), 1), (BlockId(7), 1), (BlockId(5), -1)]),
                ),
                (
                    11,
                    2,
                    Chain::from_coeffs([(BlockId(5), 1), (BlockId(9), 1), (BlockId(6), -1)]),
                ),
                (
                    12,
                    2,
                    Chain::from_coeffs([(BlockId(4), -1), (BlockId(6), 1), (BlockId(8), -1)]),
                ),
                (
                    13,
                    2,
                    Chain::from_coeffs([(BlockId(7), -1), (BlockId(8), 1), (BlockId(9), -1)]),
                ),
            ]),
            (1, 0, 1),
        );
        assert!(h.open.is_empty());
    }

    #[test]
    fn two_points_are_two_zero_zero() {
        let h = assert_betti(
            &complex(&[
                (0, 0, Chain::from_coeffs([])),
                (1, 0, Chain::from_coeffs([])),
            ]),
            (2, 0, 0),
        );
        assert!(h.open.is_empty());
    }

    /// Klein bottle: 4×4 grid of the square. Left and right are glued the same
    /// way; bottom and top are glued reversed. Each square is two triangles.
    #[test]
    fn a_triangulated_klein_bottle_is_one_one_zero() {
        let idx = |i: usize, j: usize| i * 4 + j;
        let mut parent = [0_usize; 16];
        for (i, slot) in parent.iter_mut().enumerate() {
            *slot = i;
        }
        let find = |parent: &mut [usize; 16], mut x: usize| -> usize {
            while parent[x] != x {
                let p = parent[x];
                let gp = parent[p];
                parent[x] = gp;
                x = p;
            }
            x
        };
        let unite = |parent: &mut [usize; 16], a: usize, b: usize| {
            let ra = find(parent, a);
            let rb = find(parent, b);
            if ra != rb {
                parent[rb] = ra;
            }
        };
        for j in 0..4 {
            unite(&mut parent, idx(0, j), idx(3, j));
        }
        for i in 0..4 {
            unite(&mut parent, idx(i, 0), idx(3 - i, 3));
        }
        let mut roots = BTreeSet::new();
        for i in 0..4 {
            for j in 0..4 {
                roots.insert(find(&mut parent, idx(i, j)));
            }
        }
        let mut root_id = BTreeMap::new();
        for (n, root) in roots.iter().enumerate() {
            let Ok(id) = u32::try_from(n) else {
                panic!("vertex id");
            };
            root_id.insert(*root, id);
        }
        let vid = |parent: &mut [usize; 16], i: usize, j: usize| -> u32 {
            let root = find(parent, idx(i, j));
            match root_id.get(&root) {
                Some(id) => *id,
                None => panic!("missing vertex"),
            }
        };
        let mut edge_ids: BTreeMap<(u32, u32), u32> = BTreeMap::new();
        let mut faces: Vec<BTreeMap<u32, i32>> = Vec::new();
        for si in 0..3 {
            for sj in 0..3 {
                let corners = [
                    vid(&mut parent, si, sj),
                    vid(&mut parent, si + 1, sj),
                    vid(&mut parent, si + 1, sj + 1),
                    vid(&mut parent, si, sj + 1),
                ];
                for tri in [[0_usize, 1, 2], [0, 2, 3]] {
                    let p = corners[tri[0]];
                    let q = corners[tri[1]];
                    let r = corners[tri[2]];
                    if p == q || q == r || r == p {
                        continue;
                    }
                    let mut face = BTreeMap::new();
                    for (u, v) in [(p, q), (q, r), (r, p)] {
                        let lo = u.min(v);
                        let hi = u.max(v);
                        let n = u32::try_from(edge_ids.len()).unwrap_or(0);
                        let id = *edge_ids.entry((lo, hi)).or_insert(n);
                        let sign: i32 = if u < v { 1 } else { -1 };
                        let sum = face.get(&id).copied().unwrap_or(0) + sign;
                        if sum == 0 {
                            face.remove(&id);
                        } else {
                            face.insert(id, sum);
                        }
                    }
                    if !face.is_empty() {
                        faces.push(face);
                    }
                }
            }
        }
        let nv = u32::try_from(root_id.len()).unwrap_or(0);
        let ne = u32::try_from(edge_ids.len()).unwrap_or(0);
        let mut blocks = Vec::new();
        for id in 0..nv {
            blocks.push((id, 0, Chain::from_coeffs([])));
        }
        let mut ordered: Vec<((u32, u32), u32)> = edge_ids.iter().map(|(k, v)| (*k, *v)).collect();
        ordered.sort_by_key(|(_, id)| *id);
        for ((lo, hi), local) in &ordered {
            blocks.push((nv + local, 1, edge(*hi, *lo)));
        }
        let face0 = nv + ne;
        for (n, face) in faces.iter().enumerate() {
            let Ok(id) = u32::try_from(n) else {
                continue;
            };
            let chain = Chain::from_coeffs(face.iter().map(|(local, c)| (BlockId(nv + local), *c)));
            blocks.push((face0 + id, 2, chain));
        }
        let built = complex(&blocks);
        let h = assert_betti(&built, (1, 1, 0));
        assert_eq!(h.open.len(), 1);
    }

    #[test]
    fn elimination_that_overflows_i128_names_the_block_count() {
        let big = 2_000_000_000_i32;
        let mut blocks: Vec<(u32, u32, Chain)> = Vec::new();
        for id in 0..4_u32 {
            blocks.push((id, 0, Chain::from_coeffs([])));
        }
        for id in 0..4_u32 {
            blocks.push((4 + id, 1, Chain::from_coeffs([(BlockId(id), big)])));
        }
        let built = complex(&blocks);
        match built.assemble() {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => panic!("assemble: {}", r.reason),
        }
        match built.homology() {
            Verdict::Refused(r) => {
                assert!(
                    r.reason.contains(
                        "complex too large for exact elimination at 8 blocks; acceptance is a complex whose elimination fits i128"
                    ),
                    "{}",
                    r.reason
                );
            }
            Verdict::Ok(h) => panic!("overflow must refuse, got {:?}", (h.b0, h.b1, h.b2)),
        }
    }
}
