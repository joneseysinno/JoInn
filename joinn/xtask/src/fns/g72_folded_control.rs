//! Gate 7.2 item 3 control: the subject is refused, or a link's touches miscount.

use std::collections::BTreeSet;

use joinn_visual::{Camera, ChartId, CutForm, LEVEL_MIN, Rect, UniverseLayout, Zoom, cut, touches};

use super::g72_layouts::g72_layouts;
use super::subject::Subject;
use super::zoom::VIEWPORT;

/// True when admission or a lens's layout refuses the subject, or, with each
/// lens framed at level −4 step 0, some link's touch count differs from the
/// number of distinct cut nodes its members map to (a member maps to its body
/// if drawn, else to the nearest node above it).
pub(crate) fn g72_folded_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return false;
    };
    let Ok(layouts) = g72_layouts(u) else {
        return true;
    };
    let (w, h) = VIEWPORT;
    let miscounts = |layout: &UniverseLayout| {
        let root = layout.chart(ChartId(0)).map_or((0, 0), |c| c.size);
        let framed = Camera::frame(
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
        let camera = Camera {
            zoom: Zoom {
                level: LEVEL_MIN,
                step: 0,
            },
            ..framed
        };
        let c = cut(layout, &camera);
        let node = |mut here: ChartId| loop {
            let entry = c.entries.iter().find(|e| e.chart == here);
            if entry.is_some_and(|e| matches!(e.form, CutForm::Drawn(_) | CutForm::Node)) {
                return Some(here);
            }
            match layout.chart(here).map(|ch| ch.parent) {
                Some(parent) if parent != here => here = parent,
                _ => return None,
            }
        };
        let per_link = touches(layout, &c, &u.coding.links);
        u.coding
            .links
            .iter()
            .zip(&per_link)
            .any(|(link, (_, got))| {
                let distinct: BTreeSet<ChartId> = link
                    .members
                    .iter()
                    .filter_map(|m| {
                        (0u32..)
                            .zip(&layout.charts)
                            .find(|(_, ch)| ch.name == m.body && ch.body.is_some())
                            .and_then(|(i, _)| node(ChartId(i)))
                    })
                    .collect();
                distinct.len() != got.len()
            })
    };
    layouts.iter().any(miscounts)
}
