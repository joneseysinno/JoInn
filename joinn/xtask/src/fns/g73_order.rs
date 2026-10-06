//! Gate 7.3 item 2: order is drawn only when declared; the form follows size.

use joinn_frame::Verdict;
use joinn_gpu::open;
use joinn_visual::{
    Camera, ChartId, Form, LEVEL_MIN, PieceKind, Rect, Zoom, fold_at, form_of, print_zoom,
};

use super::g6_adapters::g6_adapters;
use super::g73_form_boundary::g73_form_boundary;
use super::g73_form_fade::g73_form_fade;
use super::grove::grove_layout;
use super::links::{FORMS_BLOCK, forms_text, laid_universe};
use super::zoom::{VIEWPORT, zoom_views};

/// Every grove route is unordered, has no mid-path arrowhead, and is no spine
/// at any §2.12 view; `ordered.universe`'s `path` is a spine with a mid-path
/// arrowhead at its frame and at level −4 step 0; `links --forms` prints the
/// findings' block; a link in a form crossfade window owns its pixels in its
/// owner form (`g73_form_fade`); and on every adapter the shader's form at
/// each boundary is §2.5's (`g73_form_boundary`).
pub(crate) fn g73_order() -> bool {
    let Some(adapters) = g6_adapters() else {
        return false;
    };
    let (grove, layout) = match grove_layout() {
        Ok(g) => g,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let mut failures = Vec::new();
    let views = zoom_views(&layout);
    let (mut ordered, mut mid_path, mut spines) = (0usize, 0usize, 0usize);
    for r in &layout.routes.routes {
        ordered += usize::from(r.ordered);
        mid_path += usize::from(
            r.pieces
                .iter()
                .any(|p| p.kind == PieceKind::Arrow && p.member == 0),
        );
        spines += views
            .iter()
            .filter(|(_, c)| form_of(c.zoom, r).0 == Form::Spine)
            .count();
    }
    println!(
        "order grove: {} links, {} routes, ordered {ordered}, mid-path arrowheads {mid_path}, spines at {} views {spines}",
        grove.coding.links.len(),
        layout.routes.routes.len(),
        views.len()
    );
    if (ordered, mid_path, spines) != (0, 0, 0) {
        failures.push(format!(
            "order grove: ordered {ordered}, mid-path arrowheads {mid_path}, spines {spines}; acceptance is 0, 0, 0"
        ));
    }
    let (_, universe, small) = match laid_universe("phase5/ordered.universe") {
        Ok(l) => l,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let root = small.chart(ChartId(0)).map_or((0, 0), |c| c.size);
    let (w, h) = VIEWPORT;
    let frame = Camera::frame(
        Rect {
            x: 0,
            y: 0,
            w: root.0,
            h: root.1,
        },
        ChartId(0),
        w,
        h,
    );
    let low = Camera {
        zoom: Zoom {
            level: LEVEL_MIN,
            step: 0,
        },
        ..frame
    };
    let path = universe.coding.links.iter().position(|l| l.id == "path");
    for (label, camera) in [("frame", frame), ("framed", low)] {
        let fold = fold_at(camera.zoom);
        let route = small
            .routes
            .routes
            .iter()
            .find(|r| r.fold == fold && Some(r.link) == path);
        let Some(route) = route else {
            failures.push(format!(
                "order ordered.universe {label}: link path has no {} route",
                fold.name()
            ));
            continue;
        };
        let form = form_of(camera.zoom, route).0;
        let arrows = route
            .pieces
            .iter()
            .filter(|p| p.kind == PieceKind::Arrow && p.member == 0)
            .count();
        println!(
            "order ordered.universe {label} {} ({}): link path {}, mid-path arrowheads {arrows}",
            print_zoom(&camera.zoom),
            fold.name(),
            form.name()
        );
        if form != Form::Spine || arrows == 0 {
            failures.push(format!(
                "order ordered.universe {label}: link path is {} with {arrows} mid-path arrowhead(s); acceptance is a spine with one or more",
                form.name()
            ));
        }
    }
    match forms_text() {
        Ok(text) if text == FORMS_BLOCK => println!(
            "order grove: links --forms prints the findings' {} lines",
            FORMS_BLOCK.lines().count()
        ),
        Ok(text) => failures.push(format!(
            "links --forms differs from the findings' block:\n{text}"
        )),
        Err(e) => failures.push(e),
    }
    match g73_form_fade(&layout, &adapters) {
        Ok((lines, missed)) => {
            for line in &lines {
                println!("{line}");
            }
            failures.extend(missed);
        }
        Err(e) => {
            println!("{e}");
            return false;
        }
    }
    for adapter in &adapters {
        let gpu = match open(adapter) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => {
                println!("{}", r.reason);
                return false;
            }
        };
        match g73_form_boundary(&gpu, adapter.line()) {
            Ok((line, missed)) => {
                println!("{line}");
                failures.extend(missed);
            }
            Err(e) => {
                println!("{e}");
                return false;
            }
        }
    }
    for f in &failures {
        println!("{f}");
    }
    failures.is_empty()
}
