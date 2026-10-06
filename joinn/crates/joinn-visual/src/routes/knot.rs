//! The knot: a 1-median on the streets.

use super::Graph;

/// The node with the smallest cost, ties to the smallest `(y, x)`; `cost`
/// answers `None` for a node that cannot reach every touch point.
pub(crate) fn knot(graph: &Graph, cost: impl Fn(usize) -> Option<i64>) -> Option<usize> {
    (0..graph.nodes.len())
        .filter_map(|n| cost(n).map(|c| (c, n)))
        .min()
        .map(|(_, n)| n)
}

#[cfg(test)]
mod tests {
    use super::knot;
    use crate::routes::distances::distances;
    use crate::routes::{Line, routing_graph};

    #[test]
    fn three_members_on_a_ring_tie_and_the_smallest_y_then_x_wins() {
        // A 10 × 5 ring (perimeter 30) with touch points 10 apart: (0, 0),
        // (10, 0) and (5, 5) cost 20 each; (10, 5) and (0, 5) cost 25. Of the
        // three that tie, (0, 0) and (10, 0) have the smallest y, and (0, 0)
        // the smaller x.
        let g = routing_graph(&[
            Line {
                a: (0, 0),
                b: (10, 0),
            },
            Line {
                a: (0, 5),
                b: (10, 5),
            },
            Line {
                a: (0, 0),
                b: (0, 5),
            },
            Line {
                a: (10, 0),
                b: (10, 5),
            },
            Line {
                a: (5, 5),
                b: (5, 5),
            },
        ]);
        let dist: Vec<Vec<i64>> = [(10, 0), (5, 5), (0, 0)]
            .iter()
            .filter_map(|p| g.node_at(*p))
            .map(|t| distances(&g, t))
            .collect();
        let cost = |n: usize| Some(dist.iter().map(|d| d[n]).sum::<i64>());
        let costs: Vec<((i64, i64), i64)> = (0..g.nodes.len())
            .filter_map(|n| cost(n).map(|c| (g.nodes[n], c)))
            .collect();
        assert_eq!(
            costs,
            [
                ((0, 0), 20),
                ((10, 0), 20),
                ((0, 5), 25),
                ((5, 5), 20),
                ((10, 5), 25)
            ]
        );
        assert_eq!(knot(&g, cost).map(|n| g.nodes[n]), Some((0, 0)));
        // A spur makes (10, 0) strictly best.
        let g = routing_graph(&[
            Line {
                a: (0, 0),
                b: (10, 0),
            },
            Line {
                a: (10, 0),
                b: (10, 4),
            },
            Line {
                a: (20, 0),
                b: (10, 0),
            },
        ]);
        let ends: Vec<Vec<i64>> = [(0, 0), (10, 4), (20, 0)]
            .iter()
            .filter_map(|p| g.node_at(*p))
            .map(|t| distances(&g, t))
            .collect();
        let cost = |n: usize| Some(ends.iter().map(|d| d[n]).sum());
        assert_eq!(knot(&g, cost).map(|n| g.nodes[n]), Some((10, 0)));
    }
}
