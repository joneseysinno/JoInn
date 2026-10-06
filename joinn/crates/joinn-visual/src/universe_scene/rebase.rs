//! Re-anchor the scene to the chart under the camera's centre pixel.

use joinn_frame::Verdict;

use super::UniverseScene;
use crate::camera::Camera;
use crate::tables::Delta;

impl UniverseScene {
    /// The camera rebased by its layout (`focus ← focus − d·2^16`, pin and zoom
    /// unchanged), and the chart rows rewritten relative to the new anchor:
    /// chart rows only, at most one per chart, and none when the anchor stays.
    pub fn rebase(&mut self, camera: Camera) -> Verdict<(Camera, Delta)> {
        let next = self.layout.rebase(camera);
        if next.anchor == self.anchor {
            return Verdict::Ok((next, Delta::default()));
        }
        self.anchor = next.anchor;
        match self.write_charts() {
            Verdict::Ok(rows) => Verdict::Ok((next, Delta { rows })),
            Verdict::Refused(r) => Verdict::Refused(r),
        }
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use crate::camera::{Camera, ChartId, FOCUS_UNIT, Zoom};
    use crate::fixtures::phase5_universe;
    use crate::tables::{Table, table_bytes};
    use crate::universe_scene::UniverseScene;

    #[test]
    fn a_zoom_writes_no_row_and_the_rebase_to_calc_writes_chart_rows_only() {
        let mut scene = match UniverseScene::grow(phase5_universe(), ChartId(0)) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert!(
            !scene.tables().route.is_empty() && !scene.tables().segment.is_empty(),
            "the phase 5 link's routes are present"
        );
        let far = Camera {
            zoom: Zoom { level: -2, step: 0 },
            anchor: ChartId(0),
            focus: (2752 * FOCUS_UNIT, 800 * FOCUS_UNIT),
            pin: (960, 540),
            width: 1920,
            height: 1080,
        };
        let Verdict::Ok((same, delta)) = scene.rebase(far) else {
            panic!("rebase runs");
        };
        assert_eq!((same, delta.rows.len()), (far, 0));
        assert!(
            scene.take_pending().is_none(),
            "a camera change writes no row"
        );
        let at_s = Camera {
            zoom: Zoom { level: 3, step: 0 },
            focus: (
                113 * FOCUS_UNIT + FOCUS_UNIT * 3 / 8,
                117 * FOCUS_UNIT + FOCUS_UNIT / 4,
            ),
            ..far
        };
        let Verdict::Ok((moved, delta)) = scene.rebase(at_s) else {
            panic!("rebase runs");
        };
        assert_ne!(moved.anchor, ChartId(0));
        assert!(delta.rows.iter().all(|w| w.table == Table::Chart));
        assert!(!delta.rows.is_empty());
        assert!(delta.rows.len() <= scene.tables().chart.len());
        let fresh = match UniverseScene::grow(phase5_universe(), moved.anchor) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(table_bytes(scene.tables()), table_bytes(fresh.tables()));
    }
}
