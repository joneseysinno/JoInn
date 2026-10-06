//! Admit a universe exactly as any universe: bind, Law 4, lenses, link types,
//! assembly.

use joinn_frame::Verdict;
use joinn_link::{
    BodyStore, Universe, assemble_universe, bind, canonical_universe, check_law4, check_lenses,
    check_link_types,
};

/// The first refusal of `bind`, `check_law4`, `check_lenses`,
/// `check_link_types` and `assemble_universe`, in that order; or the admitted
/// universe in print order (`canonical_universe`), which is what anything
/// laid out from it uses.
pub(crate) fn admit_universe(universe: &Universe, store: &BodyStore) -> Verdict<Universe> {
    let bound = match bind(universe, store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    for verdict in [check_law4(universe), check_lenses(universe)] {
        if let Verdict::Refused(r) = verdict {
            return Verdict::Refused(r);
        }
    }
    if let Verdict::Refused(r) = check_link_types(universe, &bound) {
        return Verdict::Refused(r);
    }
    match assemble_universe(universe, &bound) {
        Verdict::Ok(()) => Verdict::Ok(canonical_universe(universe)),
        Verdict::Refused(r) => Verdict::Refused(r),
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_link::{Universe, parse_universe, print_universe};
    use joinn_visual::{
        Camera, ChartId, LEVEL_MIN, Pick, Rect, UniverseScene, Zoom, cpu_pick_sample,
        layout_universe, table_bytes,
    };

    use super::admit_universe;
    use crate::fns::corpus_store::corpus_store;
    use crate::fns::grove::load_universe_arg;
    use crate::fns::zoom::VIEWPORT;

    /// The Route and Segment rows and the owner of every 7th pixel, framed and
    /// at level −4, of `u` as admitted.
    fn drawn(u: &Universe) -> (Vec<u8>, Vec<u8>, Vec<String>) {
        let (_, store) = corpus_store().unwrap_or_else(|e| panic!("{e}"));
        let admitted = match admit_universe(u, store) {
            Verdict::Ok(a) => a,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let layout = match layout_universe(&admitted, store, "function") {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let root = layout.chart(ChartId(0)).map_or((0, 0), |c| c.size);
        let scene = match UniverseScene::grow(layout, ChartId(0)) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let bytes = table_bytes(scene.tables());
        let (w, h) = VIEWPORT;
        let rect = Rect {
            x: 0,
            y: 0,
            w: root.0,
            h: root.1,
        };
        let framed = Camera::frame(rect, ChartId(0), w, h);
        let low = Camera {
            zoom: Zoom {
                level: LEVEL_MIN,
                step: 0,
            },
            ..framed
        };
        let pixels: Vec<(u32, u32)> = (0..h)
            .step_by(37)
            .flat_map(|y| (0..w).step_by(37).map(move |x| (x, y)))
            .collect();
        let mut owners = Vec::new();
        for camera in [framed, low] {
            let picks = match scene
                .shapes_at(&camera)
                .and_then(|s| cpu_pick_sample(&s, &camera, &pixels))
            {
                Verdict::Ok(p) => p,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            owners.extend(picks.into_iter().map(|p| match p {
                Pick::Owned(id) => match scene.print_id(id) {
                    Verdict::Ok(n) => n,
                    Verdict::Refused(r) => r.reason,
                },
                other => format!("{other:?}"),
            }));
        }
        (bytes.route, bytes.segment, owners)
    }

    #[test]
    fn a_universe_and_its_printed_text_draw_and_pick_alike() {
        for name in [
            "grove",
            "phase5/universe.universe",
            "phase5/ordered.universe",
        ] {
            let u = load_universe_arg(name).unwrap_or_else(|e| panic!("{e}"));
            let back = match parse_universe(&print_universe(&u.coding)) {
                Verdict::Ok(b) => b,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            let (a, b) = (drawn(&u), drawn(&back));
            if name == "grove" {
                let ids = |u: &Universe| u.coding.links.iter().map(|l| l.id.clone()).collect();
                let (grown, read): (Vec<String>, Vec<String>) = (ids(&u), ids(&back));
                assert_ne!(grown, read, "the grown grove is not in print order");
                assert!(a.2.iter().any(|o| o.starts_with("link ")), "no link pixel");
            }
            assert!(!a.0.is_empty() && !a.1.is_empty(), "{name}: no route rows");
            assert_eq!(a.0, b.0, "{name}: Route rows");
            assert_eq!(a.1, b.1, "{name}: Segment rows");
            assert_eq!(a.2, b.2, "{name}: pick owners");
        }
    }
}
