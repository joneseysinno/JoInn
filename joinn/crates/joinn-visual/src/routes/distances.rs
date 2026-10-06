//! Shortest street distance from one node to every node.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use super::Graph;

/// Distances in layout units from `from`; `i64::MAX` where unreachable.
pub(crate) fn distances(graph: &Graph, from: usize) -> Vec<i64> {
    let mut dist = vec![i64::MAX; graph.nodes.len()];
    let mut heap = BinaryHeap::new();
    if let Some(d) = dist.get_mut(from) {
        *d = 0;
        heap.push(Reverse((0i64, from)));
    }
    while let Some(Reverse((d, n))) = heap.pop() {
        if d > dist[n] {
            continue;
        }
        for &(next, length) in &graph.adjacent[n] {
            let through = d + length;
            if through < dist[next] {
                dist[next] = through;
                heap.push(Reverse((through, next)));
            }
        }
    }
    dist
}
