//! Gate 7.3 item 3 control: the subject is refused, or a link touches a node
//! twice or misses a member.

use std::collections::BTreeSet;

use joinn_visual::ChartId;

use super::g72_layouts::g72_layouts;
use super::subject::Subject;

/// True when admission or a lens's layout refuses the subject, or some route
/// of some lens, in some fold state, touches one node twice, or its touches
/// do not stand for every member of its link exactly.
pub(crate) fn g73_folded_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return false;
    };
    let Ok(layouts) = g72_layouts(u) else {
        return true;
    };
    layouts.iter().any(|layout| {
        layout.routes.routes.iter().any(|r| {
            let members = u.coding.links.get(r.link).map_or(0, |l| l.members.len());
            let charts: BTreeSet<ChartId> = r.touches.iter().map(|t| t.chart).collect();
            let stood: BTreeSet<usize> = r
                .touches
                .iter()
                .flat_map(|t| t.members.iter().copied())
                .collect();
            charts.len() != r.touches.len() || stood != (0..members).collect()
        })
    })
}
