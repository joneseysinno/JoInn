//! Split streets into a graph at every crossing and every end.

use std::collections::{BTreeMap, BTreeSet};

use super::{Graph, Line};

/// Every end of every line, and every point where a line meets another
/// (crossing, T, or collinear touch), is a node; consecutive nodes along a
/// line are joined by a segment weighted by its length. A line of no length
/// is a point: it splits every line it lies on.
pub fn routing_graph(lines: &[Line]) -> Graph {
    let mut verticals: BTreeMap<i64, Vec<(i64, i64)>> = BTreeMap::new();
    let mut horizontals: BTreeMap<i64, Vec<(i64, i64)>> = BTreeMap::new();
    for l in lines {
        let ((x0, y0), (x1, y1)) = (l.a, l.b);
        if x0 == x1 {
            verticals
                .entry(x0)
                .or_default()
                .push((y0.min(y1), y0.max(y1)));
        } else if y0 == y1 {
            horizontals
                .entry(y0)
                .or_default()
                .push((x0.min(x1), x0.max(x1)));
        }
    }
    let mut points: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut pieces: Vec<Vec<(i64, i64)>> = Vec::new();
    for (&y, spans) in &horizontals {
        for &(x0, x1) in spans {
            let mut on: BTreeSet<i64> = [x0, x1].into();
            for (&x, vs) in verticals.range(x0..=x1) {
                if vs.iter().any(|&(a, b)| a <= y && y <= b) {
                    on.insert(x);
                }
            }
            for &(a, b) in spans {
                on.extend([a, b].into_iter().filter(|x| x0 <= *x && *x <= x1));
            }
            pieces.push(on.into_iter().map(|x| (x, y)).collect());
        }
    }
    for (&x, spans) in &verticals {
        for &(y0, y1) in spans {
            let mut on: BTreeSet<i64> = [y0, y1].into();
            for (&y, hs) in horizontals.range(y0..=y1) {
                if hs.iter().any(|&(a, b)| a <= x && x <= b) {
                    on.insert(y);
                }
            }
            for &(a, b) in spans {
                on.extend([a, b].into_iter().filter(|y| y0 <= *y && *y <= y1));
            }
            pieces.push(on.into_iter().map(|y| (x, y)).collect());
        }
    }
    for piece in &pieces {
        points.extend(piece.iter().copied());
    }
    let mut nodes: Vec<(i64, i64)> = points.into_iter().collect();
    nodes.sort_by_key(|p| (p.1, p.0));
    let index: BTreeMap<(i64, i64), usize> =
        nodes.iter().enumerate().map(|(i, p)| (*p, i)).collect();
    let mut edges: BTreeSet<(usize, usize)> = BTreeSet::new();
    for piece in &pieces {
        for pair in piece.windows(2) {
            if let (Some(&a), Some(&b)) = (index.get(&pair[0]), index.get(&pair[1])) {
                edges.insert((a.min(b), a.max(b)));
            }
        }
    }
    let mut adjacent: Vec<Vec<(usize, i64)>> = vec![Vec::new(); nodes.len()];
    for &(a, b) in &edges {
        let length = (nodes[a].0 - nodes[b].0).abs() + (nodes[a].1 - nodes[b].1).abs();
        adjacent[a].push((b, length));
        adjacent[b].push((a, length));
    }
    for list in &mut adjacent {
        list.sort_unstable();
    }
    Graph {
        nodes,
        edges: edges.into_iter().collect(),
        adjacent,
    }
}

#[cfg(test)]
mod tests {
    use super::routing_graph;
    use crate::routes::{
        GALAXY_GUTTER_X, GALAXY_GUTTER_Y, Line, SYSTEM_GUTTER_X, SYSTEM_GUTTER_Y,
        UNIVERSE_GUTTER_X, UNIVERSE_GUTTER_Y,
    };

    /// One level's grid lines alone, each across its container's first to
    /// last line in the other direction.
    fn grid(xs: (i64, i64, i64), ys: (i64, i64, i64)) -> Vec<Line> {
        let last = |(f, s, n): (i64, i64, i64)| f + s * (n - 1);
        let mut lines = Vec::new();
        for c in 0..xs.2 {
            let x = xs.0 + xs.1 * c;
            lines.push(Line {
                a: (x, ys.0),
                b: (x, last(ys)),
            });
        }
        for r in 0..ys.2 {
            let y = ys.0 + ys.1 * r;
            lines.push(Line {
                a: (xs.0, y),
                b: (last(xs), y),
            });
        }
        lines
    }

    #[test]
    fn a_system_has_35_crossings_a_galaxy_25_the_universe_15() {
        let count = |xs, ys| routing_graph(&grid(xs, ys)).nodes.len();
        assert_eq!(count(SYSTEM_GUTTER_X, SYSTEM_GUTTER_Y), 35);
        assert_eq!(count(GALAXY_GUTTER_X, GALAXY_GUTTER_Y), 25);
        assert_eq!(count(UNIVERSE_GUTTER_X, UNIVERSE_GUTTER_Y), 15);
        // 7 lines of 4 segments and 5 of 6.
        assert_eq!(
            routing_graph(&grid(SYSTEM_GUTTER_X, SYSTEM_GUTTER_Y))
                .edges
                .len(),
            7 * 4 + 5 * 6
        );
    }

    #[test]
    fn a_t_and_a_collinear_touch_split_lines_and_nodes_sort_by_y_then_x() {
        let g = routing_graph(&[
            Line {
                a: (0, 0),
                b: (10, 0),
            },
            Line {
                a: (4, 0),
                b: (4, 6),
            },
            Line {
                a: (10, 0),
                b: (14, 0),
            },
        ]);
        assert_eq!(g.nodes, [(0, 0), (4, 0), (10, 0), (14, 0), (4, 6)]);
        assert_eq!(g.edges, [(0, 1), (1, 2), (1, 4), (2, 3)]);
        assert_eq!(g.adjacent[1], [(0, 4), (2, 6), (4, 6)]);
        assert_eq!(g.node_at((4, 6)), Some(4));
        assert_eq!(g.node_at((5, 6)), None);
        // A point on a line splits it.
        let g = routing_graph(&[
            Line {
                a: (0, 0),
                b: (0, 8),
            },
            Line {
                a: (0, 3),
                b: (0, 3),
            },
        ]);
        assert_eq!(g.nodes, [(0, 0), (0, 3), (0, 8)]);
        assert_eq!(g.edges, [(0, 1), (1, 2)]);
    }
}
