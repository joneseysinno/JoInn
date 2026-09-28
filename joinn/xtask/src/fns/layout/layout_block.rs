//! `print_layout`, then one camera line per standard viewport.

use joinn_visual::{Layout, STANDARD_VIEWPORTS, fit, print_camera, print_layout};

/// `print_layout`, then one camera line per standard viewport.
pub(super) fn layout_block(layout: &Layout) -> String {
    let mut out = print_layout(layout);
    for (w, h) in STANDARD_VIEWPORTS {
        out.push_str(&print_camera(&fit(layout, w, h)));
        out.push('\n');
    }
    out
}
