//! Present every instance a run touched, once each, in first-appearance order.

use std::collections::BTreeSet;

use joinn_frame::Verdict;
use joinn_host::describe;
use joinn_live::{BodyState, StepReport};

use super::Scene;
use crate::tables::{Delta, RowWrite};

impl Scene {
    /// Touched: named in a report's `fired`, or the destination of a `delivered`
    /// entry (the text before the last `@`). A delivery fills an in-port before
    /// its cell fires, so destinations count.
    pub fn apply_run(&mut self, reports: &[StepReport], state: &BodyState) -> Verdict<Delta> {
        let mut touched: Vec<String> = Vec::new();
        for report in reports {
            let delivered = report
                .delivered
                .iter()
                .filter_map(|(wire, _)| wire.rsplit_once('@').map(|(dest, _)| dest.to_owned()));
            for name in report.fired.iter().cloned().chain(delivered) {
                if !touched.contains(&name) {
                    touched.push(name);
                }
            }
        }
        let mut rows: BTreeSet<RowWrite> = BTreeSet::new();
        for name in &touched {
            let d = match describe(state, name) {
                Verdict::Ok(d) => d,
                Verdict::Refused(r) => return Verdict::Refused(r),
            };
            match self.present(&d) {
                Verdict::Ok(delta) => rows.extend(delta.rows),
                Verdict::Refused(r) => return Verdict::Refused(r),
            }
        }
        Verdict::Ok(Delta {
            rows: rows.into_iter().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_link::{Address, instance_ports};
    use joinn_live::BodyState;
    use joinn_prim::sealed_natives;

    use crate::camera::{STANDARD_VIEWPORTS, fit};
    use crate::fixtures::calculator;
    use crate::scene::Scene;
    use crate::script::event;
    use crate::tables::{FILLED, REFUSED, table_bytes};

    #[test]
    fn the_script_keeps_delta_tables_equal_to_regrow_within_the_bound() {
        let (body, cells) = calculator();
        let ports = match instance_ports(&body, &cells) {
            Verdict::Ok(p) => p,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mut scene = match Scene::grow("body", &body, &cells) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(
            scene.take_pending(),
            None,
            "a grown scene has nothing pending"
        );
        let mut state = match BodyState::new(body.clone(), cells.clone(), sealed_natives(), 1) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let script = [("cli_a", "two"), ("cli_a", "2"), ("cli_b", "3")];
        for (epoch, (instance, text)) in (0u64..).zip(script) {
            let before = table_bytes(scene.tables());
            let (delta, touched) = event(
                &mut scene,
                &mut state,
                (&body, &cells),
                (instance, text),
                epoch,
            );
            let bound: usize = touched
                .iter()
                .map(|i| 1 + ports.get(i).map_or(0, Vec::len))
                .sum();
            let rows: Vec<String> = delta
                .rows
                .iter()
                .map(|w| format!("{:?} {}", w.table, w.slot))
                .collect();
            println!(
                "event {epoch} {instance}@0 <- {text:?}: touched {}; {} row(s), bound {bound}: {}",
                touched.join(", "),
                delta.rows.len(),
                rows.join(", ")
            );
            assert!(delta.rows.len() <= bound, "event {epoch}: V121 bound");
            let regrown = match Scene::regrow("body", &body, &cells, &state) {
                Verdict::Ok(s) => s,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            let after = table_bytes(scene.tables());
            assert_eq!(after, table_bytes(regrown.tables()), "event {epoch}: V122");
            assert_ne!(
                before, after,
                "event {epoch}: skipping this delta would leave tables regrow refuses"
            );
            assert_eq!(scene.take_pending().unwrap_or_default(), delta);
            assert_eq!(scene.take_pending(), None, "idle after the draw");
        }
        let sum_out = Address {
            instance: "sum".to_owned(),
            port: 2,
        };
        let filled = scene
            .ports
            .get(&sum_out)
            .and_then(|s| scene.tables.port.get(*s as usize))
            .is_some_and(|r| r.flags & FILLED != 0);
        assert!(filled, "sum@2 holds 5");
        assert!(scene.tables.cell.iter().all(|c| c.flags & REFUSED == 0));

        let bytes = table_bytes(scene.tables());
        let cameras: Vec<_> = STANDARD_VIEWPORTS
            .iter()
            .map(|(w, h)| fit(scene.layout(), *w, *h))
            .collect();
        assert_ne!(cameras[0], cameras[3]);
        assert_eq!(
            table_bytes(scene.tables()),
            bytes,
            "a camera change writes no row"
        );
        assert_eq!(
            scene.take_pending(),
            None,
            "a camera change leaves nothing pending"
        );
    }
}
