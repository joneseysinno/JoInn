//! One universe's line for one fold state.

use std::collections::BTreeSet;

use joinn_link::Link;
use joinn_visual::{Fold, Route, UniverseLayout, crossings};

/// `links <name> fold <state>: <n> links, <legs> legs, <stubs> stubs, <spines>
/// spines, knots <k>, crossings <c>`, then every crossing (§2.6) and every
/// link that touches a node twice (V151), one line each.
pub(crate) fn fold_line(
    name: &str,
    fold: Fold,
    layout: &UniverseLayout,
    links: &[Link],
    routes: &[&Route],
) -> (String, Vec<String>) {
    let mut faults = Vec::new();
    let (mut legs, mut stubs, mut spines, mut knots) = (0, 0, 0, 0);
    for r in routes {
        legs += r.legs;
        stubs += r.stubs;
        spines += usize::from(r.ordered && r.touches.len() >= 2);
        knots += usize::from(r.knot.is_some());
        faults.extend(crossings(layout, links, r));
        let charts: BTreeSet<_> = r.touches.iter().map(|t| t.chart).collect();
        if charts.len() != r.touches.len() {
            let id = links.get(r.link).map_or("?", |l| l.id.as_str());
            faults.push(format!(
                "links {name} fold {}: link {id} touches a node more than once",
                fold.name()
            ));
        }
    }
    let crossed = faults
        .iter()
        .filter(|f| !f.contains("more than once"))
        .count();
    let line = format!(
        "links {name} fold {}: {} links, {legs} legs, {stubs} stubs, {spines} spines, knots {knots}, crossings {crossed}",
        fold.name(),
        routes.len()
    );
    (line, faults)
}
