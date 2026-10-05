//! Present one body's description: its ports' filled bits, its cell's refused
//! bit, and its out-port values.

use std::collections::BTreeSet;

use joinn_dna::Direction;
use joinn_frame::Verdict;
use joinn_host::{Description, Role};
use joinn_link::Address;

use super::UniverseScene;
use crate::refuse::refuse;
use crate::tables::{
    CellRow, Delta, FILLED, PortRow, REFUSED, Row, RowWrite, STYLE_CELL, STYLE_CELL_REFUSED,
};

impl UniverseScene {
    /// A face with no port row in body `alias` is refused. The instance's cell
    /// is refused exactly when the description is a refusal. Each out-port
    /// face's value is kept, and the value strokes are rewritten from every
    /// kept value (`write_values`). Only changed rows enter the delta.
    pub fn present(&mut self, alias: &str, d: &Description) -> Verdict<Delta> {
        let Some(&body) = self.bodies.get(alias) else {
            return refuse(format!(
                "present: body {alias} is not in this scene; acceptance is a body of lens {}",
                self.layout.lens
            ));
        };
        let Some(&cell) = self.cells.get(&(body, d.instance.clone())) else {
            return refuse(format!(
                "present: instance {}.{} is not in this scene; acceptance is an instance of body {alias}",
                alias, d.instance
            ));
        };
        let mut writes: Vec<(u32, Row)> = Vec::new();
        let mut values = self.values.clone();
        for face in &d.ports {
            let address = Address {
                instance: d.instance.clone(),
                port: face.position,
            };
            let Some(&slot) = self.ports.get(&(body, address.clone())) else {
                return refuse(format!(
                    "present: port {alias}.{} is not in this scene; acceptance is a port of body {alias}",
                    address.printed()
                ));
            };
            if let Some(&old) = self.tables.port.get(slot as usize) {
                let flags = if face.value.is_some() {
                    old.flags | FILLED
                } else {
                    old.flags & !FILLED
                };
                writes.push((slot, Row::Port(PortRow { flags, ..old })));
            }
            if face.direction == Direction::Out {
                match &face.value {
                    Some(v) => values.insert((body, address), v.clone()),
                    None => values.remove(&(body, address)),
                };
            }
        }
        if let Some(&old) = self.tables.cell.get(cell as usize) {
            let refused = d.role == Role::Refusal;
            writes.push((
                cell,
                Row::Cell(CellRow {
                    flags: if refused {
                        old.flags | REFUSED
                    } else {
                        old.flags & !REFUSED
                    },
                    style: if refused {
                        STYLE_CELL_REFUSED
                    } else {
                        STYLE_CELL
                    },
                    ..old
                }),
            ));
        }
        let strokes = match self.write_values(values) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let mut changed: BTreeSet<RowWrite> = strokes.into_iter().collect();
        for (slot, row) in writes {
            if let Some(table) = self.tables.put(slot, row) {
                changed.insert(RowWrite { table, slot });
            }
        }
        self.pending.extend(changed.iter().copied());
        Verdict::Ok(Delta {
            rows: changed.into_iter().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_host::describe;
    use joinn_live::BodyState;
    use joinn_prim::sealed_natives;

    use crate::camera::ChartId;
    use crate::fixtures::{calculator, phase5_universe};
    use crate::scene::Scene;
    use crate::script::event;
    use crate::tables::{FILLED, Table, table_bytes};
    use crate::universe_scene::UniverseScene;

    #[test]
    fn a_value_writes_its_port_s_strokes_and_regrow_equals_the_deltas() {
        let (body, cells) = calculator();
        let mut driver = match Scene::grow("body", &body, &cells) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mut state = match BodyState::new(body.clone(), cells.clone(), sealed_natives(), 1) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mut scene = match UniverseScene::grow(phase5_universe(), ChartId(0)) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let strokes_before = scene.tables().stroke.len();
        let names: Vec<String> = driver
            .layout()
            .cells
            .iter()
            .map(|c| c.instance.clone())
            .collect();
        for (epoch, input) in (0u64..).zip([("cli_a", "12345"), ("cli_b", "3")]) {
            event(&mut driver, &mut state, (&body, &cells), input, epoch);
            for name in &names {
                let d = match describe(&state, name) {
                    Verdict::Ok(d) => d,
                    Verdict::Refused(r) => panic!("{}", r.reason),
                };
                if let Verdict::Refused(r) = scene.present("calc", &d) {
                    panic!("{}", r.reason);
                }
            }
        }
        let delta = scene.take_pending().unwrap_or_default();
        assert!(delta.rows.iter().any(|w| w.table == Table::Stroke));
        assert!(scene.tables().stroke.len() > strokes_before);
        let sum2 = scene
            .tables()
            .port
            .iter()
            .filter(|p| p.flags & FILLED != 0)
            .count();
        assert!(sum2 >= 1, "sum@2 is filled");
        let descriptions: Vec<(String, joinn_host::Description)> = names
            .iter()
            .rev()
            .map(|n| match describe(&state, n) {
                Verdict::Ok(d) => ("calc".to_owned(), d),
                Verdict::Refused(r) => panic!("{}", r.reason),
            })
            .collect();
        let regrown = match UniverseScene::regrow(phase5_universe(), ChartId(0), &descriptions) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(table_bytes(scene.tables()), table_bytes(regrown.tables()));
    }
}
