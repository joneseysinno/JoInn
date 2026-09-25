//! The boundary of one block, as a chain one dimension down.

use joinn_frame::{CheckId, Refusal, Verdict};

use crate::BlockId;
use crate::chain::Chain;

use super::Complex;

impl Complex {
    /// The boundary of one block. An unknown block is refused by its id.
    pub fn boundary(&self, block: BlockId) -> Verdict<Chain> {
        if !self.dimension.contains_key(&block) {
            return Verdict::Refused(Refusal::structural(
                CheckId::Other,
                format!(
                    "block {} is not in this complex; acceptance is a block that was added",
                    block.0
                ),
            ));
        }
        match self.boundaries.get(&block) {
            Some(chain) => Verdict::Ok(chain.clone()),
            None => Verdict::Refused(Refusal::structural(
                CheckId::Other,
                format!(
                    "block {} has no boundary chain; acceptance is a chain, empty for a 0-block",
                    block.0
                ),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use std::collections::BTreeMap;

    use crate::{BlockId, Chain, Complex};

    #[test]
    fn boundary_of_an_edge_is_head_with_tail_negated() {
        let tail = BlockId(0);
        let head = BlockId(1);
        let edge = BlockId(2);
        let complex = Complex {
            dimension: BTreeMap::from([(tail, 0), (head, 0), (edge, 1)]),
            boundaries: BTreeMap::from([(edge, Chain::from_coeffs([(head, 1), (tail, -1)]))]),
        };
        match complex.boundary(edge) {
            Verdict::Ok(chain) => {
                assert_eq!(chain, Chain::from_coeffs([(head, 1), (tail, -1)]));
            }
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }
}
