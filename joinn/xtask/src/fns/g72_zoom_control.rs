//! Gate 7.2 item 1 control: the subject is refused, or has no `function` lens.

use super::g72_layouts::g72_layouts;
use super::subject::Subject;

/// True when admission or a lens's layout refuses the subject, or no laid-out
/// lens is `function`.
pub(crate) fn g72_zoom_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return false;
    };
    match g72_layouts(u) {
        Ok(layouts) => !layouts.iter().any(|l| l.lens == "function"),
        Err(_) => true,
    }
}
