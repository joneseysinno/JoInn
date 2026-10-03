//! Re-anchor a camera to the chart under its centre pixel.

use super::UniverseLayout;
use crate::camera::{Camera, FOCUS_UNIT};

impl UniverseLayout {
    /// When the anchor changes to a chart whose origin is `d` from the old
    /// one's, `focus ← focus − d·2^16`. Pin, zoom and every pixel stay exactly
    /// the same.
    pub fn rebase(&self, camera: Camera) -> Camera {
        let anchor = self.anchor_of(&camera);
        if anchor == camera.anchor {
            return camera;
        }
        let from = self.chart(camera.anchor).map_or((0, 0), |c| c.root_origin);
        let to = self.chart(anchor).map_or((0, 0), |c| c.root_origin);
        Camera {
            anchor,
            focus: (
                camera.focus.0 - (to.0 - from.0) * FOCUS_UNIT,
                camera.focus.1 - (to.1 - from.1) * FOCUS_UNIT,
            ),
            ..camera
        }
    }
}

#[cfg(test)]
mod tests {
    use joinn_dna::{hash, parse_body, parse_cell};
    use joinn_frame::{FrameRegistry, Verdict};
    use joinn_link::{BodyStore, parse_universe};

    use crate::camera::{Camera, ChartId, FOCUS_UNIT, Zoom};
    use crate::charts::{UniverseLayout, layout_universe};
    use crate::fixtures::calculator;

    fn phase5() -> UniverseLayout {
        let (body, mut cells) = calculator();
        if let Verdict::Ok(c) = parse_cell(
            include_str!("../../../../corpus/phase21/mul.cell"),
            &FrameRegistry::phase1(),
        ) {
            cells.insert(hash(&c.coding), c);
        }
        let units = match parse_body(
            include_str!("../../../../corpus/phase5/units.body"),
            &FrameRegistry::phase1(),
        ) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mut store = BodyStore::new();
        for (b, src) in [(body, "calculator.body"), (units, "units.body")] {
            if let Verdict::Refused(r) = store.insert(b, cells.clone(), src) {
                panic!("{}", r.reason);
            }
        }
        let u = match parse_universe(include_str!("../../../../corpus/phase5/universe.universe")) {
            Verdict::Ok(u) => u,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        match layout_universe(&u, &store, "function") {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    fn at_s(level: i32) -> Camera {
        Camera {
            zoom: Zoom { level, step: 0 },
            anchor: ChartId(0),
            focus: (
                113 * FOCUS_UNIT + FOCUS_UNIT * 3 / 8,
                117 * FOCUS_UNIT + FOCUS_UNIT / 4,
            ),
            pin: (960, 540),
            width: 1920,
            height: 1080,
        }
    }

    #[test]
    fn the_s_centre_anchors_the_calc_body_and_the_rebase_moves_no_pixel() {
        let l = phase5();
        let c = at_s(3);
        let r = l.rebase(c);
        let Some(chart) = l.chart(r.anchor) else {
            panic!("no chart {:?}", r.anchor);
        };
        assert_eq!(chart.name, "calc");
        assert_eq!(chart.root_origin, (88, 112));
        assert_eq!(
            r.focus,
            (
                25 * FOCUS_UNIT + FOCUS_UNIT * 3 / 8,
                5 * FOCUS_UNIT + FOCUS_UNIT / 4
            )
        );
        assert_eq!((r.pin, r.zoom), (c.pin, c.zoom));
        for (u, v) in [(0i64, 0i64), (24, 4), (40, 24), (-88, -112)] {
            let in_body = (u * FOCUS_UNIT, v * FOCUS_UNIT);
            let in_root = ((u + 88) * FOCUS_UNIT, (v + 112) * FOCUS_UNIT);
            assert_eq!(r.pixel_of(in_body), c.pixel_of(in_root));
        }
        assert_eq!(l.rebase(r), r);
    }

    #[test]
    fn a_centre_between_galaxies_anchors_the_universe() {
        let l = phase5();
        assert_eq!(l.chart_at((2752, 800), 1), ChartId(0));
        assert_eq!(l.chart_at((64, 64), 1).0, 1);
        assert_eq!(l.chart_at((64 + 1296, 64), 1), ChartId(0));
    }
}
