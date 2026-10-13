//! Every node of an order.

use joinn_frame::Verdict;

use super::{Point, point};
use crate::dim::{Dim, check_dim};

/// Nodes 0 … 4ⁿ − 1 in curve order; a dimension other than 2 is refused.
pub fn nodes(dim: Dim, order: u32) -> Verdict<Vec<Point>> {
    if let Verdict::Refused(r) = check_dim(dim) {
        return Verdict::Refused(r);
    }
    Verdict::Ok((0..1u64 << (2 * order)).map(|d| point(order, d)).collect())
}
