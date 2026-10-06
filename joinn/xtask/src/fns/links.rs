//! `cargo xtask links`: the routing graph and the routes of plan 7.3 §2.2–2.6.

use joinn_visual::{Fold, grid_lines, routing_graph};

use super::grove::grove_layout;

/// The grove's open routing graph: `graph grove: <nodes> nodes, <segments>
/// segments` (grid lines and extensions, before any stub).
pub(crate) fn links(args: Vec<String>) -> Result<(), String> {
    if !args.is_empty() {
        return Err("usage: cargo xtask links".into());
    }
    let (_, layout) = grove_layout()?;
    let graph = routing_graph(&grid_lines(&layout, Fold::Open));
    println!(
        "graph grove: {} nodes, {} segments",
        graph.nodes.len(),
        graph.edges.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use joinn_frame::Verdict;
    use joinn_visual::{Fold, routes};

    use crate::fns::grove::grove_layout;

    #[test]
    fn every_grove_link_folded_touches_each_node_once() {
        let (grove, layout) = grove_layout().unwrap_or_else(|e| panic!("{e}"));
        let all = match routes(&layout, &grove.coding.links) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let folded: Vec<_> = all
            .routes
            .iter()
            .filter(|r| r.fold == Fold::Systems)
            .collect();
        assert_eq!(folded.len(), grove.coding.links.len());
        let mut touches = 0;
        for r in &folded {
            let charts: BTreeSet<_> = r.touches.iter().map(|t| t.chart).collect();
            assert_eq!(
                charts.len(),
                r.touches.len(),
                "link {} touches a node twice",
                r.link
            );
            let members: usize = r.touches.iter().map(|t| t.members.len()).sum();
            assert_eq!(members, grove.coding.links[r.link].members.len());
            touches += r.touches.len();
        }
        assert_eq!(touches, 264);
    }
}
