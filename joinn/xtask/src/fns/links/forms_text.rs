//! Everything `cargo xtask links --forms` prints.

use std::fmt::Write;

use joinn_frame::Verdict;
use joinn_visual::{Form, fold_at, form_of, print_zoom, routes};

use crate::fns::grove::grove_layout;
use crate::fns::zoom::zoom_views;

/// Per §2.12 view of the grove: `forms <label> <zoom> (<fold>): region <a>,
/// hub <b>, bundle <c>, spine <d>, fading <f>`, counting the routes of the
/// fold state the view shows that draw anything, by owner form; `fading`
/// counts those in a form crossfade window.
pub(crate) fn forms_text() -> Result<String, String> {
    let (grove, layout) = grove_layout()?;
    let all = match routes(&layout, &grove.coding.links) {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => return Err(format!("grove: {}", r.reason)),
    };
    let mut out = String::new();
    for (label, camera) in zoom_views(&layout) {
        let fold = fold_at(camera.zoom);
        let mut counts = [0usize; 4];
        let mut fading = 0usize;
        for route in all
            .routes
            .iter()
            .filter(|r| r.fold == fold && !r.pieces.is_empty())
        {
            let (form, window) = form_of(camera.zoom, route);
            counts[form.number() as usize] += 1;
            fading += usize::from(window.is_some());
        }
        let _ = writeln!(
            out,
            "forms {label} {} ({}): {} {}, {} {}, {} {}, {} {}, fading {fading}",
            print_zoom(&camera.zoom),
            fold.name(),
            Form::Region.name(),
            counts[0],
            Form::Hub.name(),
            counts[1],
            Form::Bundle.name(),
            counts[2],
            Form::Spine.name(),
            counts[3],
        );
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_visual::{Form, PieceKind, form_of, routes};

    use crate::fns::grove::grove_layout;
    use crate::fns::zoom::zoom_views;

    #[test]
    fn no_grove_link_is_a_spine_or_has_a_mid_path_arrowhead_at_any_view() {
        let (grove, layout) = grove_layout().unwrap_or_else(|e| panic!("{e}"));
        let all = match routes(&layout, &grove.coding.links) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let views = zoom_views(&layout);
        for route in &all.routes {
            assert!(!route.ordered);
            assert!(
                !route
                    .pieces
                    .iter()
                    .any(|p| p.kind == PieceKind::Arrow && p.member == 0),
                "link {} has a mid-path arrowhead",
                route.link
            );
            for (label, camera) in &views {
                assert_ne!(form_of(camera.zoom, route).0, Form::Spine, "{label}");
            }
        }
    }
}
