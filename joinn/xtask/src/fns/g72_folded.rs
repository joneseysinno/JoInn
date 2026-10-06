//! Gate 7.2 item 3: a folded system is one node, and a link touches it once.

use joinn_link::Universe;
use joinn_visual::{
    Camera, ChartId, ChartKind, CutForm, LEVEL_MIN, Rect, UniverseLayout, Zoom, cut, touches,
};

use super::g72_layouts::g72_layouts;
use super::grove::{grove_layout, load_universe_arg};
use super::zoom::{VIEWPORT, zoom_views};

/// Two systems of one body each, one ordered link between them.
const ORDERED: &str = "phase5/ordered.universe";

/// On the grove at the `s`, level −4 step 0: 128 system nodes, no body drawn,
/// 264 touches; framed: 1182 touches. On `ordered.universe` framed at level
/// −4 step 0: each of its two systems is one node and link `path` touches 2
/// nodes.
pub(crate) fn g72_folded() -> bool {
    let mut failures = Vec::new();
    let total = |layout: &UniverseLayout, universe: &Universe, camera: &Camera| -> usize {
        let c = cut(layout, camera);
        touches(layout, &c, &universe.coding.links)
            .iter()
            .map(|(_, nodes)| nodes.len())
            .sum()
    };
    let (grove, layout) = match grove_layout() {
        Ok(g) => g,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let views = zoom_views(&layout);
    let low = views
        .iter()
        .find(|(l, c)| l == "at s" && c.zoom.level == LEVEL_MIN && c.zoom.step == 0);
    let framed = views.iter().find(|(l, _)| l == "frame");
    let (Some((_, low)), Some((_, framed))) = (low, framed) else {
        println!("the grove's views have no frame or no level −4 view at the s");
        return false;
    };
    let c = cut(&layout, low);
    let kind = |id: ChartId| layout.chart(id).map(|ch| ch.kind);
    let nodes = c
        .entries
        .iter()
        .filter(|e| e.form == CutForm::Node && kind(e.chart) == Some(ChartKind::System))
        .count();
    let drawn = c
        .entries
        .iter()
        .filter(|e| matches!(e.form, CutForm::Drawn(_)))
        .count();
    if (nodes, drawn) != (128, 0) {
        failures.push(format!(
            "grove at the s, level −4 step 0: {nodes} system nodes, {drawn} bodies drawn; acceptance is 128 and 0"
        ));
    }
    for (what, camera, want) in [("level −4 step 0", low, 264), ("the frame", framed, 1182)] {
        let got = total(&layout, &grove, camera);
        if got != want {
            failures.push(format!(
                "grove at {what}: {got} touches; acceptance is {want}"
            ));
        }
    }
    let ordered = load_universe_arg(ORDERED).and_then(|u| {
        let layouts = g72_layouts(&u)?;
        Ok((u, layouts))
    });
    let (universe, layouts) = match ordered {
        Ok(a) => a,
        Err(e) => {
            println!("{ORDERED}: {e}");
            return false;
        }
    };
    let (w, h) = VIEWPORT;
    for layout in &layouts {
        let root = layout.chart(ChartId(0)).map_or((0, 0), |ch| ch.size);
        let camera = Camera {
            zoom: Zoom {
                level: LEVEL_MIN,
                step: 0,
            },
            ..Camera::frame(
                Rect {
                    x: 0,
                    y: 0,
                    w: root.0,
                    h: root.1,
                },
                ChartId(0),
                w,
                h,
            )
        };
        let c = cut(layout, &camera);
        let systems: Vec<(String, Option<CutForm>)> = (0u32..)
            .zip(&layout.charts)
            .filter(|(_, ch)| ch.kind == ChartKind::System)
            .map(|(i, ch)| {
                let form = c
                    .entries
                    .iter()
                    .find(|e| e.chart == ChartId(i))
                    .map(|e| e.form);
                (ch.name.clone(), form)
            })
            .collect();
        if systems.len() != 2 || systems.iter().any(|(_, f)| *f != Some(CutForm::Node)) {
            failures.push(format!(
                "{ORDERED} lens {} at level −4: systems {systems:?}; acceptance is two, each one node",
                layout.lens
            ));
        }
        let path = touches(layout, &c, &universe.coding.links)
            .into_iter()
            .find(|(id, _)| id == "path")
            .map_or(0, |(_, nodes)| nodes.len());
        if path != 2 {
            failures.push(format!(
                "{ORDERED} lens {}: link path touches {path} nodes; acceptance is 2",
                layout.lens
            ));
        }
    }
    for f in &failures {
        println!("{f}");
    }
    failures.is_empty()
}
