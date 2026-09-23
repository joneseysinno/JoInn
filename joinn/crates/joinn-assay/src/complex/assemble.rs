//! ∂∂ = 0 on every block. A refusal names the block.

use joinn_frame::{CheckId, Refusal, Verdict};
use std::collections::BTreeMap;

use crate::BlockId;

use super::Complex;

impl Complex {
    /// Every block's boundary lands on the dimension below, and ∂∂ is empty.
    pub fn assemble(&self) -> Verdict<()> {
        let refuse = |reason: String| -> Verdict<()> {
            Verdict::Refused(Refusal::structural(CheckId::Other, reason))
        };
        let mut ids: Vec<BlockId> = self.dimension.keys().copied().collect();
        ids.sort();
        for id in ids {
            let dim = match self.dimension.get(&id) {
                Some(d) => *d,
                None => continue,
            };
            let boundary = match self.boundaries.get(&id) {
                Some(c) => c,
                None => {
                    return refuse(format!(
                        "block {} has no boundary chain; acceptance is a chain, empty for a 0-block",
                        id.0
                    ));
                }
            };
            if dim == 0 {
                if !boundary.coeff.is_empty() {
                    return refuse(format!(
                        "block {} is a 0-block with a boundary; acceptance is an empty chain",
                        id.0
                    ));
                }
                continue;
            }
            for face in boundary.coeff.keys() {
                match self.dimension.get(face) {
                    None if dim == 1 => {
                        return refuse(format!(
                            "block {} endpoint {} is not a 0-block of this complex; acceptance is a 0-block",
                            id.0, face.0
                        ));
                    }
                    None => {
                        return refuse(format!(
                            "block {} boundary names block {} which is not in this complex; acceptance is a block that was added",
                            id.0, face.0
                        ));
                    }
                    Some(face_dim) if *face_dim + 1 != dim => {
                        return refuse(format!(
                            "block {} boundary names block {} of dimension {face_dim}; acceptance is dimension {}",
                            id.0,
                            face.0,
                            dim - 1
                        ));
                    }
                    Some(_) => {}
                }
            }
            if dim >= 2 {
                let mut acc: BTreeMap<BlockId, i32> = BTreeMap::new();
                for (face, coeff) in &boundary.coeff {
                    let Some(face_boundary) = self.boundaries.get(face) else {
                        continue;
                    };
                    for (vertex, face_coeff) in &face_boundary.coeff {
                        let sum = acc.get(vertex).copied().unwrap_or(0) + coeff * face_coeff;
                        if sum == 0 {
                            acc.remove(vertex);
                        } else {
                            acc.insert(*vertex, sum);
                        }
                    }
                }
                if !acc.is_empty() {
                    return Verdict::Refused(Refusal::structural(
                        CheckId::Other,
                        format!(
                            "block {} is not closed; acceptance is a boundary whose own boundary is empty",
                            id.0
                        ),
                    ));
                }
            }
        }
        Verdict::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use std::collections::BTreeMap;

    use crate::{BlockId, Chain, Complex};

    fn complex(blocks: &[(BlockId, u32, Chain)]) -> Complex {
        let mut dimension = BTreeMap::new();
        let mut boundaries = BTreeMap::new();
        for (id, dim, chain) in blocks {
            dimension.insert(*id, *dim);
            boundaries.insert(*id, chain.clone());
        }
        Complex {
            dimension,
            boundaries,
        }
    }

    fn edge() -> Complex {
        let tail = BlockId(0);
        let head = BlockId(1);
        let span = BlockId(2);
        complex(&[
            (tail, 0, Chain::from_coeffs([])),
            (head, 0, Chain::from_coeffs([])),
            (span, 1, Chain::from_coeffs([(head, 1), (tail, -1)])),
        ])
    }

    #[test]
    fn a_closed_triangle_assembles() {
        let v0 = BlockId(0);
        let v1 = BlockId(1);
        let v2 = BlockId(2);
        let e01 = BlockId(3);
        let e12 = BlockId(4);
        let e20 = BlockId(5);
        let face = BlockId(6);
        let built = complex(&[
            (v0, 0, Chain::from_coeffs([])),
            (v1, 0, Chain::from_coeffs([])),
            (v2, 0, Chain::from_coeffs([])),
            (e01, 1, Chain::from_coeffs([(v1, 1), (v0, -1)])),
            (e12, 1, Chain::from_coeffs([(v2, 1), (v1, -1)])),
            (e20, 1, Chain::from_coeffs([(v0, 1), (v2, -1)])),
            (
                face,
                2,
                Chain::from_coeffs([(e01, 1), (e12, 1), (e20, 1)]),
            ),
        ]);
        match built.assemble() {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn a_closed_edge_assembles() {
        match edge().assemble() {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn an_endpoint_that_is_not_a_0_block_is_refused() {
        let tail = BlockId(0);
        let span = BlockId(2);
        let built = complex(&[
            (tail, 0, Chain::from_coeffs([])),
            (span, 1, Chain::from_coeffs([(BlockId(9), 1), (tail, -1)])),
        ]);
        match built.assemble() {
            Verdict::Refused(r) => {
                assert!(r.reason.contains("block 2"), "{}", r.reason);
                assert!(r.reason.contains("9"), "{}", r.reason);
                assert!(r.reason.contains("0-block"), "{}", r.reason);
            }
            Verdict::Ok(()) => panic!("a missing endpoint must refuse"),
        }
    }

    #[test]
    fn a_2_block_that_is_not_closed_is_refused() {
        let a = BlockId(0);
        let b = BlockId(1);
        let span = BlockId(2);
        let face = BlockId(3);
        let built = complex(&[
            (a, 0, Chain::from_coeffs([])),
            (b, 0, Chain::from_coeffs([])),
            (span, 1, Chain::from_coeffs([(b, 1), (a, -1)])),
            (face, 2, Chain::from_coeffs([(span, 1)])),
        ]);
        match built.assemble() {
            Verdict::Refused(r) => {
                assert!(r.reason.contains("block 3"), "{}", r.reason);
                assert!(r.reason.contains("closed") || r.reason.contains("empty"), "{}", r.reason);
            }
            Verdict::Ok(()) => panic!("an open face must refuse"),
        }
    }

    #[test]
    fn a_boundary_of_the_wrong_dimension_is_refused() {
        let point = BlockId(0);
        let span = BlockId(1);
        let other = BlockId(2);
        let built = complex(&[
            (point, 0, Chain::from_coeffs([])),
            (other, 1, Chain::from_coeffs([(point, 1)])),
            (span, 1, Chain::from_coeffs([(other, 1), (point, -1)])),
        ]);
        match built.assemble() {
            Verdict::Refused(r) => {
                assert!(r.reason.contains("block 1"), "{}", r.reason);
                assert!(r.reason.contains("dimension"), "{}", r.reason);
            }
            Verdict::Ok(()) => panic!("a boundary one dimension too high must refuse"),
        }
    }
}
