//! A shortest path, with the plan's tie-break.

use super::Graph;

/// The nodes of a shortest path from `from` to the node `to_target` measures
/// from: at each node, of the neighbours one step closer, the one with the
/// smallest `(y, x)` (the smallest index). Empty when unreachable.
pub(crate) fn walk(graph: &Graph, to_target: &[i64], from: usize) -> Vec<usize> {
    if to_target.get(from).is_none_or(|d| *d == i64::MAX) {
        return Vec::new();
    }
    let mut path = vec![from];
    let mut here = from;
    while to_target[here] > 0 {
        let next = graph.adjacent[here]
            .iter()
            .filter(|(n, length)| {
                to_target[*n] != i64::MAX && to_target[*n] + length == to_target[here]
            })
            .map(|(n, _)| *n)
            .min();
        match next {
            Some(n) => {
                path.push(n);
                here = n;
            }
            None => return Vec::new(),
        }
    }
    path
}
