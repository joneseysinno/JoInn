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
