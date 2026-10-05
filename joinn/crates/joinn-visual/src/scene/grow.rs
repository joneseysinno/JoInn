//! Grow a scene from DNA: every row placed in canonical order, nothing pending.

use std::collections::{BTreeMap, BTreeSet};

use joinn_dna::{Body, Cell};
use joinn_frame::{Hash, Verdict};

use super::Scene;
use crate::layout::layout;
use crate::tables::Tables;

impl Scene {
    /// Cells in name order, then each cell's ports by position, then wires in
    /// `print_body` order. Two grows of one body give the same bytes.
    pub fn grow(alias: &str, body: &Body, cells: &BTreeMap<Hash, Cell>) -> Verdict<Scene> {
        let placed = match layout(body, cells) {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let mut scene = Scene {
            alias: alias.to_owned(),
            layout: placed.clone(),
            tables: Tables::empty(),
            cells: BTreeMap::new(),
            ports: BTreeMap::new(),
            links: BTreeMap::new(),
            interior: BTreeSet::new(),
            pending: BTreeSet::new(),
        };
        if let Verdict::Refused(r) = scene.place(placed) {
            return Verdict::Refused(r);
        }
        scene.pending.clear();
        Verdict::Ok(scene)
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use crate::fixtures::calculator;
    use crate::scene::Scene;
    use crate::tables::{CHART_BODY, ChartRow, LIVE};

    #[test]
    fn a_single_body_scene_has_one_chart_row_the_body_s_at_origin_0() {
        let (body, cells) = calculator();
        let scene = match Scene::grow("body", &body, &cells) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(
            scene.tables().chart,
            [ChartRow {
                origin_x: 0,
                origin_y: 0,
                parent: 0,
                kind: CHART_BODY,
                size: 40,
                generation: 1,
                flags: LIVE,
            }]
        );
        assert!(scene.tables().frame.is_empty());
        assert!(scene.tables().stroke.is_empty());
    }
}
