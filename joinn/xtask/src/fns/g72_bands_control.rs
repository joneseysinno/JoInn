//! Gate 7.2 item 2 control: the subject is refused, or a lens draws a body twice.

use std::collections::BTreeSet;

use joinn_visual::ChartKind;

use super::g72_layouts::g72_layouts;
use super::subject::Subject;

/// True when admission or a lens's layout refuses the subject, or some body
/// alias has two body charts in one lens.
pub(crate) fn g72_bands_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return false;
    };
    let Ok(layouts) = g72_layouts(u) else {
        return true;
    };
    layouts.iter().any(|layout| {
        let mut seen = BTreeSet::new();
        layout
            .charts
            .iter()
            .filter(|c| c.kind == ChartKind::Body)
            .any(|c| !seen.insert(c.name.as_str()))
    })
}
