//! Gate 7.3 item 1 control: the subject is refused, or a route crosses.

use joinn_visual::crossings;

use super::g72_layouts::g72_layouts;
use super::subject::Subject;

/// True when admission or a lens's layout refuses the subject, or some route
/// of some lens, in some fold state, breaks one of §2.6's four checks.
pub(crate) fn g73_touch_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return false;
    };
    let Ok(layouts) = g72_layouts(u) else {
        return true;
    };
    layouts.iter().any(|layout| {
        layout
            .routes
            .routes
            .iter()
            .any(|r| !crossings(layout, &u.coding.links, r).is_empty())
    })
}
