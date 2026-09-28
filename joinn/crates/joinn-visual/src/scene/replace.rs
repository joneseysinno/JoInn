//! Replace the body a scene draws, keeping every slot that still names something.

use std::collections::BTreeMap;

use joinn_dna::{Body, Cell};
use joinn_frame::{Hash, Verdict};

use super::Scene;
use crate::layout::layout;
use crate::tables::Delta;

impl Scene {
    /// A kept instance keeps its slot and generation and is rewritten only if it
    /// moved. A gone instance's cell, port and link slots are freed. A new one
    /// takes free slots. The body row is rewritten if the membrane changed.
    pub fn replace(&mut self, body: &Body, cells: &BTreeMap<Hash, Cell>) -> Verdict<Delta> {
        match layout(body, cells) {
            Verdict::Ok(next) => self.place(next),
            Verdict::Refused(r) => Verdict::Refused(r),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use joinn_frame::Verdict;

    use crate::camera::{STANDARD_VIEWPORTS, fit};
    use crate::dropped::dropped;
    use crate::fixtures::calculator;
    use crate::pick::{Pick, cpu_pick, shapes_of_layout};
    use crate::scene::Scene;
    use crate::tables::shapes_of_tables;

    #[test]
    fn replace_to_drop_cli_b_writes_five_rows_and_stales_its_old_id() {
        let (body, cells) = calculator();
        let mut scene = match Scene::grow("body", &body, &cells) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let (old_cli_a, old_cli_b) = ([1, 1, 0, 1], [1, 2, 0, 1]);
        match scene.resolve(old_cli_b) {
            Verdict::Ok(o) => assert_eq!(scene.print_owner(&o), "body.cli_b"),
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
        let delta = match scene.replace(&dropped(&body, "cli_b"), &cells) {
            Verdict::Ok(d) => d,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let rows: Vec<String> = delta
            .rows
            .iter()
            .map(|w| format!("{:?} {}", w.table, w.slot))
            .collect();
        println!(
            "replace to DropGenome(cli_b): {} row(s): {}",
            delta.rows.len(),
            rows.join(", ")
        );
        assert_eq!(delta.rows.len(), 5, "§2.7 predicts 5 rows");
        match scene.resolve(old_cli_b) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "stale pick: cell slot 1 generation 1, now 2; acceptance is a pick taken from the current picture"
            ),
            Verdict::Ok(o) => panic!("stale cli_b resolved to {}", scene.print_owner(&o)),
        }
        match scene.resolve(old_cli_a) {
            Verdict::Ok(o) => assert_eq!(scene.print_owner(&o), "body.cli_a"),
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
        assert_eq!(scene.take_pending().unwrap_or_default(), delta);
    }

    #[test]
    fn after_replace_every_pixel_owner_equals_a_fresh_grow() {
        let (body, cells) = calculator();
        let next = dropped(&body, "cli_b");
        let mut scene = match Scene::grow("body", &body, &cells) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        if let Verdict::Refused(r) = scene.replace(&next, &cells) {
            panic!("{}", r.reason);
        }
        let fresh = match Scene::grow("body", &next, &cells) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(scene.layout(), fresh.layout());
        for (w, h) in STANDARD_VIEWPORTS {
            let camera = fit(scene.layout(), w, h);
            let owners = |s: &Scene| -> Vec<String> {
                let image = match cpu_pick(&shapes_of_tables(s.tables()), &camera) {
                    Verdict::Ok(i) => i,
                    Verdict::Refused(r) => panic!("{}", r.reason),
                };
                let mut named: BTreeMap<[u32; 4], String> = BTreeMap::new();
                image
                    .pixels
                    .iter()
                    .map(|p| match p {
                        Pick::Background => "background".to_owned(),
                        Pick::Edge => "edge".to_owned(),
                        Pick::Owned(id) => named
                            .entry(*id)
                            .or_insert_with(|| match s.resolve(*id) {
                                Verdict::Ok(o) => s.print_owner(&o),
                                Verdict::Refused(r) => panic!("{}", r.reason),
                            })
                            .clone(),
                    })
                    .collect()
            };
            let (replaced, grown) = (owners(&scene), owners(&fresh));
            let differ = replaced.iter().zip(&grown).filter(|(a, b)| a != b).count();
            println!(
                "after replace {w}x{h}: {} pixels, {differ} owner(s) differ from a fresh grow",
                replaced.len()
            );
            assert_eq!(differ, 0, "{w}x{h}");
        }
    }

    #[test]
    fn a_position_holding_two_ports_is_refused_not_merged() {
        let src = "body {\n  codex 1\n  genome {\n    prim:eq as a\n  }\n  grants { }\n  wires { }\n  budget { steps 1 }\n  lineage none\n}\n";
        let body = match joinn_dna::parse_body(src, &joinn_frame::FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        match Scene::grow("body", &body, &BTreeMap::new()) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "scene: a@0 names an in-port and an out-port; acceptance is a body whose cells give each position one port, since an ID names a port by position"
            ),
            Verdict::Ok(_) => panic!("two ports at a@0 must be refused"),
        }
    }

    #[test]
    fn a_grown_scene_draws_the_shapes_of_its_layout() {
        let (body, cells) = calculator();
        let scene = match Scene::grow("body", &body, &cells) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(
            shapes_of_tables(scene.tables()),
            shapes_of_layout(scene.layout())
        );
    }
}
