//! One link's route in one fold state (plan 7.3 §2.3–2.4).

use std::collections::BTreeMap;

use joinn_frame::Verdict;
use joinn_link::{Link, Mark, Order};

use super::distances::distances;
use super::knot::knot;
use super::side::side;
use super::stub::stub;
use super::walk::walk;
use super::{
    Fold, GALAXY_SIDES, Graph, Line, Piece, PieceKind, Route, SIXTEENTHS, SYSTEM_SIDES, Touch,
};
use crate::camera::ChartId;
use crate::charts::{ChartKind, UniverseLayout};
use crate::refuse::refuse;

/// Members map to their body (open) or the folded system or galaxy holding it;
/// members under one folded node share its one touch point (V151). A drawn
/// member's touch point is its port's centre and its leg ends where its stub
/// meets the gutter. A folded node's touch point is the side facing the knot
/// (for a spine, facing the node's neighbour in member order). An unordered
/// link with two touch points or more has a knot and one leg per touch point;
/// an ordered one is a spine through its touch points in member order, with an
/// arrowhead at the middle of each step. Every tail's stub carries an
/// arrowhead pointing away from its port. `cache` holds distances from graph
/// nodes already measured.
pub(crate) fn route_link(
    layout: &UniverseLayout,
    graph: &Graph,
    fold: Fold,
    (index, link): (usize, &Link),
    cache: &mut BTreeMap<usize, Vec<i64>>,
) -> Verdict<Route> {
    let body_of = |alias: &str| {
        (0u32..)
            .zip(&layout.charts)
            .find(|(_, c)| c.kind == ChartKind::Body && c.name == alias)
            .map(|(i, _)| ChartId(i))
    };
    let parent = |id: ChartId| layout.chart(id).map_or(id, |c| c.parent);
    let mut touches: Vec<Touch> = Vec::new();
    let mut sequence: Vec<usize> = Vec::new();
    let mut stubs: Vec<(usize, Vec<Line>)> = Vec::new();
    for (m, member) in link.members.iter().enumerate() {
        let Some(body) = body_of(&member.body) else {
            continue;
        };
        let node = match fold {
            Fold::Open => body,
            Fold::Systems => parent(body),
            Fold::Galaxies => parent(parent(body)),
        };
        let t = if fold == Fold::Open {
            let Some(chart) = layout.chart(body) else {
                continue;
            };
            let Some(laid) = chart.body.and_then(|(_, li)| layout.layouts.get(li)) else {
                return refuse(format!(
                    "routes: body {} has no layout; acceptance is a laid-out body",
                    member.body
                ));
            };
            let Some(port) = laid
                .ports
                .iter()
                .find(|p| p.address.instance == member.instance && p.address.port == member.port)
            else {
                return refuse(format!(
                    "routes: link {} member {}.{}@{} has no port in the layout; acceptance is a declared port",
                    link.id, member.body, member.instance, member.port
                ));
            };
            let system = layout.chart(chart.parent).map_or((0, 0), |c| c.root_origin);
            let lines = stub(chart, system, laid, port);
            let point = lines.first().map_or((0, 0), |l| l.a);
            let end = lines.last().map_or(point, |l| l.b);
            stubs.push((m, lines));
            touches.push(Touch {
                chart: body,
                point,
                members: vec![m],
                node: end,
            });
            touches.len() - 1
        } else if let Some(t) = touches.iter().position(|t| t.chart == node) {
            touches[t].members.push(m);
            t
        } else {
            touches.push(Touch {
                chart: node,
                point: (0, 0),
                members: vec![m],
                node: (0, 0),
            });
            touches.len() - 1
        };
        if sequence.last() != Some(&t) {
            sequence.push(t);
        }
    }

    // Each touch's candidate graph nodes: its stub end, or a folded node's
    // four sides (left, right, top, bottom).
    let mut candidates: Vec<Vec<usize>> = Vec::new();
    for t in &touches {
        let points: Vec<(i64, i64)> = match (fold, layout.chart(t.chart)) {
            (Fold::Open, _) => vec![t.node],
            (_, Some(c)) => {
                let sides = if c.kind == ChartKind::Galaxy {
                    GALAXY_SIDES
                } else {
                    SYSTEM_SIDES
                };
                sides
                    .iter()
                    .map(|s| (c.root_origin.0 + s.0, c.root_origin.1 + s.1))
                    .collect()
            }
            (_, None) => Vec::new(),
        };
        let mut nodes = Vec::new();
        for p in points {
            match graph.node_at(p) {
                Some(n) => nodes.push(n),
                None => {
                    return refuse(format!(
                        "routes: link {} touch point {p:?} is not on the streets; acceptance is a point of the routing graph",
                        link.id
                    ));
                }
            }
        }
        for &n in &nodes {
            cache.entry(n).or_insert_with(|| distances(graph, n));
        }
        candidates.push(nodes);
    }
    let chosen = |t: usize, toward: (i64, i64)| -> usize {
        let nodes = &candidates[t];
        if nodes.len() == 1 {
            return nodes[0];
        }
        let (origin, size) = layout
            .chart(touches[t].chart)
            .map_or(((0, 0), (0, 0)), |c| (c.root_origin, c.size));
        nodes[side(origin, size, toward).min(nodes.len() - 1)]
    };
    let centre = |t: usize| {
        layout.chart(touches[t].chart).map_or((0, 0), |c| {
            (
                c.root_origin.0 + c.size.0 / 2,
                c.root_origin.1 + c.size.1 / 2,
            )
        })
    };

    let s = SIXTEENTHS;
    let at = |n: usize| (graph.nodes[n].0 * s, graph.nodes[n].1 * s);
    // Consecutive collinear steps of a path, merged into pieces.
    let merge = |path: &[usize], member: u32, legs: u32, out: &mut Vec<Piece>| {
        let mut run: Option<(usize, usize)> = None;
        for pair in path.windows(2) {
            let (a, b) = (pair[0], pair[1]);
            let along = |start: usize| {
                graph.nodes[start].0 == graph.nodes[b].0 || graph.nodes[start].1 == graph.nodes[b].1
            };
            run = match run {
                Some((start, _)) if along(start) => Some((start, b)),
                Some((start, end)) => {
                    out.push(Piece {
                        a: at(start),
                        b: at(end),
                        kind: PieceKind::Leg,
                        member,
                        legs,
                    });
                    Some((a, b))
                }
                None => Some((a, b)),
            };
        }
        if let Some((start, end)) = run {
            out.push(Piece {
                a: at(start),
                b: at(end),
                kind: PieceKind::Leg,
                member,
                legs,
            });
        }
    };
    // An arrowhead two units long at the middle of a polyline (sixteenths).
    let middle_arrow = |points: &[(i64, i64)], member: u32| -> Option<Piece> {
        let total: i64 = points
            .windows(2)
            .map(|w| (w[1].0 - w[0].0).abs() + (w[1].1 - w[0].1).abs())
            .sum();
        if total == 0 {
            return None;
        }
        let mut left = total / 2;
        for w in points.windows(2) {
            let length = (w[1].0 - w[0].0).abs() + (w[1].1 - w[0].1).abs();
            if length > 0 && left <= length {
                let d = ((w[1].0 - w[0].0).signum(), (w[1].1 - w[0].1).signum());
                let mid = (w[0].0 + d.0 * left, w[0].1 + d.1 * left);
                return Some(Piece {
                    a: (mid.0 - d.0 * s, mid.1 - d.1 * s),
                    b: (mid.0 + d.0 * s, mid.1 + d.1 * s),
                    kind: PieceKind::Arrow,
                    member,
                    legs: 1,
                });
            }
            left -= length;
        }
        None
    };

    let mut pieces: Vec<Piece> = Vec::new();
    let mut knot_at: Option<(i64, i64)> = None;
    let mut legs = 0;
    let ordered = link.order == Order::Ordered;
    if ordered && sequence.len() >= 2 {
        let mut node_of: Vec<Option<usize>> = vec![None; touches.len()];
        for (i, &t) in sequence.iter().enumerate() {
            if node_of[t].is_none() {
                let toward = sequence
                    .get(i + 1)
                    .or_else(|| i.checked_sub(1).and_then(|p| sequence.get(p)))
                    .map_or(centre(t), |&o| centre(o));
                node_of[t] = Some(chosen(t, toward));
            }
        }
        for pair in sequence.windows(2) {
            let (Some(from), Some(to)) = (node_of[pair[0]], node_of[pair[1]]) else {
                continue;
            };
            let Some(to_dist) = cache.get(&to) else {
                continue;
            };
            let path = walk(graph, to_dist, from);
            if path.is_empty() {
                return refuse(format!(
                    "routes: link {} has no street between two of its touch points; acceptance is a connected routing graph",
                    link.id
                ));
            }
            merge(&path, 0, 1, &mut pieces);
            let points: Vec<(i64, i64)> = path.iter().map(|&n| at(n)).collect();
            pieces.extend(middle_arrow(&points, 0));
        }
        if fold != Fold::Open {
            for (t, n) in node_of.iter().enumerate() {
                if let Some(n) = n {
                    touches[t].point = graph.nodes[*n];
                    touches[t].node = graph.nodes[*n];
                }
            }
        }
    } else if !ordered && touches.len() >= 2 {
        let cost = |n: usize| -> Option<i64> {
            let mut sum = 0i64;
            for t in 0..touches.len() {
                let d = cache.get(&chosen(t, graph.nodes[n]))?.get(n).copied()?;
                if d == i64::MAX {
                    return None;
                }
                sum += d;
            }
            Some(sum)
        };
        let Some(k) = knot(graph, cost) else {
            return refuse(format!(
                "routes: link {} has no node that reaches every touch point; acceptance is a connected routing graph",
                link.id
            ));
        };
        let targets: Vec<usize> = (0..touches.len())
            .map(|t| chosen(t, graph.nodes[k]))
            .collect();
        let paths: Vec<Vec<usize>> = targets
            .iter()
            .map(|target| cache.get(target).map_or(Vec::new(), |d| walk(graph, d, k)))
            .collect();
        if fold != Fold::Open {
            for (t, target) in targets.iter().enumerate() {
                touches[t].point = graph.nodes[*target];
                touches[t].node = graph.nodes[*target];
            }
        }
        let mut sharing: BTreeMap<(usize, usize), u32> = BTreeMap::new();
        for path in &paths {
            for w in path.windows(2) {
                *sharing.entry((w[0].min(w[1]), w[0].max(w[1]))).or_default() += 1;
            }
        }
        // Each step drawn once: split every leg where its sharing count or
        // direction changes, skipping steps an earlier leg drew.
        let mut drawn: BTreeMap<(usize, usize), ()> = BTreeMap::new();
        for (t, path) in paths.iter().enumerate() {
            let only = match touches[t].members.as_slice() {
                [m] => u32::try_from(m + 1).unwrap_or(0),
                _ => 0,
            };
            let mut run: Vec<usize> = Vec::new();
            let mut run_key: Option<(u32, u32)> = None;
            for w in path.windows(2) {
                let edge = (w[0].min(w[1]), w[0].max(w[1]));
                let count = sharing.get(&edge).copied().unwrap_or(1);
                let key = (if count == 1 { only } else { 0 }, count);
                let fresh = drawn.insert(edge, ()).is_none();
                if !fresh || run_key != Some(key) {
                    if let Some((member, legs_here)) = run_key {
                        merge(&run, member, legs_here, &mut pieces);
                    }
                    run.clear();
                    run_key = None;
                }
                if fresh {
                    if run.is_empty() {
                        run.push(w[0]);
                    }
                    run.push(w[1]);
                    run_key = Some(key);
                }
            }
            if let Some((member, legs_here)) = run_key {
                merge(&run, member, legs_here, &mut pieces);
            }
        }
        knot_at = Some(graph.nodes[k]);
        legs = touches.len();
    }
    for (m, lines) in &stubs {
        let member = u32::try_from(m + 1).unwrap_or(0);
        for l in lines {
            pieces.push(Piece {
                a: (l.a.0 * s, l.a.1 * s),
                b: (l.b.0 * s, l.b.1 * s),
                kind: PieceKind::Stub,
                member,
                legs: 1,
            });
        }
        if link.members.get(*m).is_some_and(|x| x.mark == Mark::Tail) {
            let mut points: Vec<(i64, i64)> =
                lines.iter().map(|l| (l.a.0 * s, l.a.1 * s)).collect();
            points.extend(lines.last().map(|l| (l.b.0 * s, l.b.1 * s)));
            pieces.extend(middle_arrow(&points, member));
        }
    }
    if let Some(k) = knot_at {
        pieces.push(Piece {
            a: (k.0 * s, k.1 * s),
            b: (k.0 * s, k.1 * s),
            kind: PieceKind::Knot,
            member: 0,
            legs: 0,
        });
    }
    let mut box_points: Vec<(i64, i64)> = touches.iter().map(|t| t.point).collect();
    box_points.extend(knot_at);
    let (xs, ys): (Vec<i64>, Vec<i64>) = box_points.iter().copied().unzip();
    let extent = |v: &[i64]| {
        v.iter()
            .max()
            .zip(v.iter().min())
            .map_or(0, |(hi, lo)| hi - lo)
    };
    Verdict::Ok(Route {
        link: index,
        fold,
        ordered,
        stubs: stubs.len(),
        touches,
        knot: knot_at,
        legs,
        size: extent(&xs).max(extent(&ys)),
        pieces,
    })
}
