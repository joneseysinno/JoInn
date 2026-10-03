//! Touches: which cut nodes each link reaches. Data, not drawn.

use joinn_link::Link;
use std::collections::BTreeMap;

use super::{Cut, CutForm};
use crate::camera::ChartId;
use crate::charts::{ChartKind, UniverseLayout};

/// For each link, in order, the distinct cut nodes its members map to: a
/// member's body if the body is drawn, otherwise the lens node that holds it.
/// A member under neither (off screen) touches nothing. Each node appears at
/// most once per link (V146).
pub fn touches(layout: &UniverseLayout, cut: &Cut, links: &[Link]) -> Vec<(String, Vec<ChartId>)> {
    let forms: BTreeMap<ChartId, CutForm> = cut.entries.iter().map(|e| (e.chart, e.form)).collect();
    let body_of: BTreeMap<&str, ChartId> = (0u32..)
        .zip(&layout.charts)
        .filter(|(_, c)| c.kind == ChartKind::Body)
        .map(|(i, c)| (c.name.as_str(), ChartId(i)))
        .collect();
    let mut out = Vec::new();
    for link in links {
        let mut nodes: Vec<ChartId> = Vec::new();
        for member in &link.members {
            let Some(mut here) = body_of.get(member.body.as_str()).copied() else {
                continue;
            };
            let node = loop {
                if matches!(forms.get(&here), Some(CutForm::Drawn(_) | CutForm::Node)) {
                    break Some(here);
                }
                match layout.chart(here).map(|c| c.parent) {
                    Some(parent) if parent != here => here = parent,
                    _ => break None,
                }
            };
            if let Some(n) = node {
                if !nodes.contains(&n) {
                    nodes.push(n);
                }
            }
        }
        out.push((link.id.clone(), nodes));
    }
    out
}

#[cfg(test)]
mod tests {
    use joinn_dna::{hash, parse_body};
    use joinn_frame::{FrameRegistry, Verdict};
    use joinn_link::{BodyStore, Universe, parse_universe};

    use super::touches;
    use crate::camera::{Camera, ChartId, FOCUS_UNIT, Zoom};
    use crate::charts::layout_universe;
    use crate::fixtures::calculator;
    use crate::lens_cut::cut;

    fn store() -> BodyStore {
        let (body, cells) = calculator();
        let mut store = BodyStore::new();
        let mut all = cells.clone();
        if let Verdict::Ok(c) = joinn_dna::parse_cell(
            include_str!("../../../../corpus/phase21/mul.cell"),
            &FrameRegistry::phase1(),
        ) {
            all.insert(hash(&c.coding), c);
        }
        if let Verdict::Refused(r) = store.insert(body, all.clone(), "calculator.body") {
            panic!("{}", r.reason);
        }
        let units = match parse_body(
            include_str!("../../../../corpus/phase5/units.body"),
            &FrameRegistry::phase1(),
        ) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        if let Verdict::Refused(r) = store.insert(units, all, "units.body") {
            panic!("{}", r.reason);
        }
        store
    }

    fn universe() -> Universe {
        match parse_universe(include_str!("../../../../corpus/phase5/universe.universe")) {
            Verdict::Ok(u) => u,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    fn touched(lens: &str, level: i32, focus: (i64, i64)) -> Vec<String> {
        let u = universe();
        let l = match layout_universe(&u, &store(), lens) {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let camera = Camera {
            zoom: Zoom { level, step: 0 },
            anchor: ChartId(0),
            focus,
            pin: (0, 0),
            width: 1920,
            height: 1080,
        };
        let c = cut(&l, &camera);
        touches(&l, &c, &u.coding.links)
            .into_iter()
            .flat_map(|(_, nodes)| nodes)
            .filter_map(|n| l.chart(n).map(|c| c.name.clone()))
            .collect()
    }

    #[test]
    fn members_map_to_their_system_node_when_systems_are_too_small_to_open() {
        assert_eq!(
            touched("function", -4, (0, 0)),
            ["calculation", "measurement"]
        );
    }

    #[test]
    fn two_members_under_one_node_touch_it_once() {
        assert_eq!(touched("deployment", -4, (0, 0)), ["local"]);
    }

    #[test]
    fn drawn_bodies_are_touched_themselves() {
        assert_eq!(touched("function", 0, (0, 0)), ["calc", "units"]);
    }

    #[test]
    fn members_off_screen_touch_nothing() {
        assert!(touched("function", 0, (-4096 * FOCUS_UNIT, 0)).is_empty());
    }
}
