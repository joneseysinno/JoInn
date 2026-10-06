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

/// `cargo xtask links --forms`, the grove's form counts at each §2.12 view
/// as `docs/Findings/phase-7.3-links.md` records them.
pub(crate) const FORMS_BLOCK: &str = "\
forms frame level -2 step 95 (k 351/1024) (open): region 128, hub 9, bundle 0, spine 0, fading 0
forms at s level -4 step 0 (k 1/16) (systems folded): region 9, hub 0, bundle 0, spine 0, fading 0
forms at s level -3 step 0 (k 1/8) (open): region 136, hub 1, bundle 0, spine 0, fading 0
forms at s level -2 step 0 (k 1/4) (open): region 136, hub 1, bundle 0, spine 0, fading 8
forms at s level -1 step 0 (k 1/2) (open): region 128, hub 9, bundle 0, spine 0, fading 1
forms at s level 0 step 0 (k 1) (open): region 127, hub 9, bundle 1, spine 0, fading 89
forms at s level 1 step 0 (k 2) (open): region 2, hub 134, bundle 1, spine 0, fading 8
forms at s level 2 step 0 (k 4) (open): region 0, hub 128, bundle 9, spine 0, fading 0
forms at s level 3 step 0 (k 8) (open): region 0, hub 127, bundle 10, spine 0, fading 89
forms at s level 4 step 0 (k 16) (open): region 0, hub 2, bundle 135, spine 0, fading 0
forms at s level 5 step 0 (k 32) (open): region 0, hub 0, bundle 137, spine 0, fading 0
forms at s level 6 step 0 (k 64) (open): region 0, hub 0, bundle 137, spine 0, fading 0
forms at s level 7 step 0 (k 128) (open): region 0, hub 0, bundle 137, spine 0, fading 0
forms at s level 8 step 0 (k 256) (open): region 0, hub 0, bundle 137, spine 0, fading 0
forms at s level 9 step 0 (k 512) (open): region 0, hub 0, bundle 137, spine 0, fading 0
";

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
