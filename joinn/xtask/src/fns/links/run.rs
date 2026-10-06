//! `cargo xtask links [--universe <path|grove> | --forms | --measure]`.

use joinn_frame::Verdict;
use joinn_visual::{Fold, grid_lines, routes, routing_graph};

use super::plants::plants;
use super::{LINK_UNIVERSES, fold_line, forms_text, laid_universe, measure_text};
use crate::fns::grove::grove_layout;

/// Per universe (default `universe.universe`, `ordered.universe`, the grove)
/// and fold state, its routes' line and every crossing; the grove's open grid
/// graph line; the standing plants' lines (rule 93); then `links: <u>
/// universes, crossings 0, each folded node touched once; planted crossing,
/// double touch, false arrow: refused (ok)`. A crossing, a node touched twice,
/// a false arrowhead or a plant not refused fails. `--forms`: the
/// grove's form counts per §2.12 view instead; `--measure`: routing's wall
/// time per universe.
pub(crate) fn links(args: Vec<String>) -> Result<(), String> {
    let names: Vec<String> = match args.as_slice() {
        [] => LINK_UNIVERSES.iter().map(|s| (*s).to_owned()).collect(),
        [flag, name] if flag == "--universe" => vec![name.clone()],
        [flag] if flag == "--forms" => {
            print!("{}", forms_text()?);
            return Ok(());
        }
        [flag] if flag == "--measure" => {
            print!("{}", measure_text()?);
            return Ok(());
        }
        _ => {
            return Err(
                "usage: cargo xtask links [--universe <path|grove> | --forms | --measure]".into(),
            );
        }
    };
    let mut faults = Vec::new();
    for arg in &names {
        let (name, universe, layout) = laid_universe(arg)?;
        let all = match routes(&layout, &universe.coding.links) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => return Err(format!("{name}: {}", r.reason)),
        };
        for fold in &all.folds {
            let of_fold: Vec<_> = all.routes.iter().filter(|r| r.fold == *fold).collect();
            let (line, found) = fold_line(&name, *fold, &layout, &universe.coding.links, &of_fold);
            println!("{line}");
            for f in &found {
                println!("{f}");
            }
            faults.extend(found);
        }
    }
    let (grove, layout) = grove_layout()?;
    if names.iter().any(|n| n == "grove") {
        let graph = routing_graph(&grid_lines(&layout, Fold::Open));
        println!(
            "graph grove: {} nodes, {} segments",
            graph.nodes.len(),
            graph.edges.len()
        );
    }
    let (planted, missed) = plants(&layout, &grove.coding.links);
    for line in &planted {
        println!("{line}");
    }
    if !faults.is_empty() {
        return Err(format!("links: {} fault(s)", faults.len()));
    }
    if !missed.is_empty() {
        return Err(format!(
            "links: planted {} not refused; acceptance is refused (rule 93)",
            missed.join(", ")
        ));
    }
    println!(
        "links: {} universes, crossings 0, each folded node touched once; planted crossing, double touch, false arrow: refused (ok)",
        names.len()
    );
    Ok(())
}
