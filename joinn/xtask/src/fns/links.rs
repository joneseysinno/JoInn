//! `cargo xtask links`: the routing graph and the routes of plan 7.3 §2.2–2.6.

mod fold_line;
mod forms_text;
mod laid_universe;
mod measure_text;
mod run;

pub(crate) use fold_line::fold_line;
pub(crate) use forms_text::forms_text;
pub(crate) use laid_universe::laid_universe;
pub(crate) use measure_text::measure_text;
pub(crate) use run::links;

/// The universes `links` reports by default, in order.
pub(crate) const LINK_UNIVERSES: [&str; 3] = [
    "phase5/universe.universe",
    "phase5/ordered.universe",
    "grove",
];

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
