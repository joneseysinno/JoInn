//! One universe's line for one fold state.

use std::collections::BTreeSet;

use joinn_link::{Link, Order};
use joinn_visual::{Fold, PieceKind, Route, UniverseLayout, crossings};

/// `links <name> fold <state>: <n> links, <legs> legs, <stubs> stubs, <spines>
/// spines, knots <k>, crossings <c>`, then every crossing (§2.6), every link
/// that touches a node twice (V151) and every mid-path arrowhead on a link
/// that declares no order (V150), one line each.
pub(crate) fn fold_line(
    name: &str,
    fold: Fold,
    layout: &UniverseLayout,
    links: &[Link],
    routes: &[&Route],
) -> (String, Vec<String>) {
    let mut faults = Vec::new();
    let mut crossed = 0;
    let (mut legs, mut stubs, mut spines, mut knots) = (0, 0, 0, 0);
    for r in routes {
        legs += r.legs;
        stubs += r.stubs;
        spines += usize::from(r.ordered && r.touches.len() >= 2);
        knots += usize::from(r.knot.is_some());
        let found = crossings(layout, links, r);
        crossed += found.len();
        faults.extend(found);
        let link = links.get(r.link);
        let id = link.map_or("?", |l| l.id.as_str());
        let charts: BTreeSet<_> = r.touches.iter().map(|t| t.chart).collect();
        if charts.len() != r.touches.len() {
            faults.push(format!(
                "links {name} fold {}: link {id} touches a node more than once",
                fold.name()
            ));
        }
        let mid_path = r
            .pieces
            .iter()
            .any(|p| p.kind == PieceKind::Arrow && p.member == 0);
        if mid_path && link.is_none_or(|l| l.order != Order::Ordered) {
            faults.push(format!(
                "links {name} fold {}: link {id} has a mid-path arrowhead and declares no order",
                fold.name()
            ));
        }
    }
    let line = format!(
        "links {name} fold {}: {} links, {legs} legs, {stubs} stubs, {spines} spines, knots {knots}, crossings {crossed}",
        fold.name(),
        routes.len()
    );
    (line, faults)
}
