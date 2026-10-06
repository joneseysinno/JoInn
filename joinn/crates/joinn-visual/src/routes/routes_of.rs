//! Every link's routes, grown once per fold state.

use std::collections::BTreeMap;

use joinn_frame::Verdict;
use joinn_link::Link;

use super::grid_lines::grid_lines;
use super::route_link::route_link;
use super::routing_graph::routing_graph;
use super::stub::stub;
use super::{Fold, Line, Routes};
use crate::bands::{Band, owner_band};
use crate::camera::{LEVEL_MIN, Zoom};
use crate::charts::{ChartKind, GALAXY_SIZE, SYSTEM_SIZE, UniverseLayout};

/// The fold states the zoom range reaches (open always; systems or galaxies
/// folded when, at the lowest zoom, their owner band is below Summary), each
/// with its routing graph (the open one with every member's stub end as a
/// node), and every link's route in each.
pub fn routes(layout: &UniverseLayout, links: &[Link]) -> Verdict<Routes> {
    let lowest = Zoom {
        level: LEVEL_MIN,
        step: 0,
    };
    let folds_at = |size: (i64, i64)| owner_band(lowest, size.0.max(size.1)) < Band::Summary;
    let mut folds = vec![Fold::Open];
    if folds_at(SYSTEM_SIZE) {
        folds.push(Fold::Systems);
    }
    if folds_at(GALAXY_SIZE) {
        folds.push(Fold::Galaxies);
    }
    let mut out = Routes {
        folds: folds.clone(),
        ..Routes::default()
    };
    for fold in folds {
        let mut lines = grid_lines(layout, fold);
        if fold == Fold::Open {
            for member in links.iter().flat_map(|l| &l.members) {
                let found = layout
                    .charts
                    .iter()
                    .find(|c| c.kind == ChartKind::Body && c.name == member.body);
                let Some(body) = found else {
                    continue;
                };
                let Some(laid) = body.body.and_then(|(_, li)| layout.layouts.get(li)) else {
                    continue;
                };
                let Some(port) = laid.ports.iter().find(|p| {
                    p.address.instance == member.instance && p.address.port == member.port
                }) else {
                    continue;
                };
                let system = layout.chart(body.parent).map_or((0, 0), |c| c.root_origin);
                if let Some(end) = stub(body, system, laid, port).last() {
                    lines.push(Line { a: end.b, b: end.b });
                }
            }
        }
        let graph = routing_graph(&lines);
        let mut cache: BTreeMap<usize, Vec<i64>> = BTreeMap::new();
        for (i, link) in links.iter().enumerate() {
            match route_link(layout, &graph, fold, (i, link), &mut cache) {
                Verdict::Ok(r) => out.routes.push(r),
                Verdict::Refused(r) => return Verdict::Refused(r),
            }
        }
        out.graphs.push(graph);
    }
    Verdict::Ok(out)
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_link::{Link, Mark, Member, Order};

    use super::routes;
    use crate::fixtures::phase5_universe;
    use crate::routes::{Fold, PieceKind, SYSTEM_SIDES};

    #[test]
    fn three_members_in_one_folded_system_have_one_touch_point_and_one_leg() {
        let layout = phase5_universe();
        let port_of = |alias: &str, n: usize| {
            let chart = layout.charts.iter().find(|c| c.name == alias);
            let laid = chart
                .and_then(|c| c.body)
                .and_then(|(_, li)| layout.layouts.get(li));
            let p = laid
                .and_then(|l| l.ports.get(n))
                .unwrap_or_else(|| panic!("{alias} port {n}"));
            Member {
                body: alias.to_owned(),
                instance: p.address.instance.clone(),
                port: p.address.port,
                mark: Mark::None,
            }
        };
        let link = Link {
            id: "trio".to_owned(),
            order: Order::None,
            members: vec![
                port_of("calc", 0),
                port_of("calc", 1),
                port_of("calc", 2),
                port_of("units", 0),
            ],
        };
        let all = match routes(&layout, &[link]) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(all.folds, [Fold::Open, Fold::Systems]);
        let open = &all.routes[0];
        assert_eq!((open.touches.len(), open.legs, open.stubs), (4, 4, 4));
        assert!(open.knot.is_some());
        let folded = &all.routes[1];
        assert_eq!(folded.fold, Fold::Systems);
        let members: Vec<Vec<usize>> = folded.touches.iter().map(|t| t.members.clone()).collect();
        assert_eq!(members, [vec![0, 1, 2], vec![3]]);
        assert_eq!((folded.legs, folded.stubs), (2, 0));
        assert!(!folded.pieces.iter().any(|p| p.kind == PieceKind::Stub));
        // Each touch point is a side of its system, the side facing the knot.
        let knot = folded
            .knot
            .unwrap_or_else(|| panic!("two nodes have a knot"));
        for t in &folded.touches {
            let c = layout.chart(t.chart).unwrap_or_else(|| panic!("chart"));
            let s = crate::routes::side(c.root_origin, c.size, knot);
            let want = (
                c.root_origin.0 + SYSTEM_SIDES[s].0,
                c.root_origin.1 + SYSTEM_SIDES[s].1,
            );
            assert_eq!(t.point, want, "{}", c.name);
        }
    }

    #[test]
    fn an_ordered_link_is_a_spine_with_no_knot_and_a_tail_arrow() {
        let layout = phase5_universe();
        let u = match joinn_link::parse_universe(include_str!(
            "../../../../corpus/phase5/universe.universe"
        )) {
            Verdict::Ok(u) => u,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let all = match routes(&layout, &u.coding.links) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let open = &all.routes[0];
        assert!(open.ordered && open.knot.is_none());
        assert_eq!((open.legs, open.stubs, open.touches.len()), (0, 2, 2));
        let arrows = open.pieces.iter().filter(|p| p.kind == PieceKind::Arrow);
        // One at the spine's middle, one on calc.sum@2's (the tail's) stub.
        assert_eq!(arrows.map(|p| p.member).collect::<Vec<_>>(), [0, 1]);
    }
}
