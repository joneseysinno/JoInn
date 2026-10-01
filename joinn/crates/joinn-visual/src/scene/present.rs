//! Present one description: its ports' filled bits, and the refused bits.

use std::collections::BTreeSet;

use joinn_dna::Direction;
use joinn_frame::Verdict;
use joinn_host::{Description, Role};
use joinn_link::Address;

use super::Scene;
use crate::refuse::refuse;
use crate::tables::{
    CellRow, Delta, FILLED, LATENT, PortRow, REFUSED, Row, RowWrite, STYLE_CELL, STYLE_CELL_REFUSED,
};

impl Scene {
    /// A port is filled when its face holds a value. A face at one of a
    /// contact's interior ports (a member port, a response in-port) is skipped;
    /// any other face with no port row is refused (V134). A refusal marks its
    /// own cell refused; any other description clears every refused bit, as
    /// `BodyState` clears `last_refusal` when anything fires. A response is
    /// latent while no out-port face holds a value. Only changed rows enter the delta.
    pub fn present(&mut self, d: &Description) -> Verdict<Delta> {
        let Some(&cell) = self.cells.get(&d.instance) else {
            return refuse(format!(
                "present: instance {} is not in this scene; acceptance is an instance of body {}",
                d.instance, self.alias
            ));
        };
        let mut writes: Vec<(u32, Row)> = Vec::new();
        for face in &d.ports {
            let address = Address {
                instance: d.instance.clone(),
                port: face.position,
            };
            let Some(&slot) = self.ports.get(&address) else {
                if self.interior.contains(&address) {
                    continue;
                }
                return refuse(format!(
                    "present: port {} is not in this scene; acceptance is a port of body {}",
                    address.printed(),
                    self.alias
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
        }
        let refused = d.role == Role::Refusal;
        let response = self
            .layout
            .cells
            .iter()
            .any(|c| c.response && c.instance == d.instance);
        let latent = !d
            .ports
            .iter()
            .any(|f| f.direction == Direction::Out && f.value.is_some());
        for &slot in self.cells.values() {
            let Some(&old) = self.tables.cell.get(slot as usize) else {
                continue;
            };
            let old = match (slot == cell && response, latent) {
                (true, true) => CellRow {
                    flags: old.flags | LATENT,
                    ..old
                },
                (true, false) => CellRow {
                    flags: old.flags & !LATENT,
                    ..old
                },
                (false, _) => old,
            };
            if refused && slot == cell {
                writes.push((
                    slot,
                    Row::Cell(CellRow {
                        flags: old.flags | REFUSED,
                        style: STYLE_CELL_REFUSED,
                        ..old
                    }),
                ));
            } else if !refused {
                writes.push((
                    slot,
                    Row::Cell(CellRow {
                        flags: old.flags & !REFUSED,
                        style: STYLE_CELL,
                        ..old
                    }),
                ));
            }
        }
        let mut changed: BTreeSet<RowWrite> = BTreeSet::new();
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
    use std::collections::BTreeMap;

    use joinn_dna::{hash, parse_cell, parse_contact};
    use joinn_frame::{FrameRegistry, Verdict};
    use joinn_host::describe;
    use joinn_link::{Address, lower};
    use joinn_live::BodyState;
    use joinn_prim::sealed_natives;

    use crate::fixtures::calculator;
    use crate::scene::Scene;
    use crate::script::event;
    use crate::tables::{RowWrite, Table};

    #[test]
    fn the_wired_calculator_refuses_a_port_it_does_not_hold() {
        let (body, cells) = calculator();
        let mut scene = match Scene::grow("body", &body, &cells) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let state = match BodyState::new(body, cells, sealed_natives(), 1) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mut d = match describe(&state, "sum") {
            Verdict::Ok(d) => d,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let Some(mut extra) = d.ports.first().cloned() else {
            panic!("sum has faces");
        };
        extra.position = 9;
        d.ports.push(extra);
        match scene.present(&d) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "present: port sum@9 is not in this scene; acceptance is a port of body body"
            ),
            Verdict::Ok(_) => panic!("sum@9 is not a port of the wired calculator"),
        }
        assert!(
            scene.take_pending().is_none(),
            "a refused present writes no row"
        );
    }

    #[test]
    fn the_contact_calculator_skips_interior_ports_and_writes_only_sum_2() {
        let frames = FrameRegistry::phase1();
        let mut cells = BTreeMap::new();
        for src in [
            include_str!("../../../../corpus/phase0/sum.cell"),
            include_str!("../../../../corpus/phase0/cli_input.cell"),
        ] {
            match parse_cell(src, &frames) {
                Verdict::Ok(c) => {
                    cells.insert(hash(&c.coding), c);
                }
                Verdict::Refused(r) => panic!("{}", r.reason),
            }
        }
        let contact = match parse_contact(
            include_str!("../../../../corpus/phase7/calculator.contact"),
            &frames,
        ) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let body = match lower(&contact, &cells, &frames) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let grow = || match Scene::grow_contact("body", &contact, &cells) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let (mut scene, mut driver) = (grow(), grow());
        let mut state = match BodyState::new(body.clone(), cells.clone(), sealed_natives(), 1) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        for (epoch, input) in (0u64..).zip([("cli_a", "2"), ("cli_b", "3")]) {
            event(&mut driver, &mut state, (&body, &cells), input, epoch);
        }
        let d = match describe(&state, "sum") {
            Verdict::Ok(d) => d,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let faces: Vec<String> = d
            .ports
            .iter()
            .map(|f| format!("sum@{} {}", f.position, f.value.as_deref().unwrap_or("-")))
            .collect();
        assert_eq!(faces, ["sum@0 2", "sum@1 3", "sum@2 5"]);
        let delta = match scene.present(&d) {
            Verdict::Ok(delta) => delta,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let sum2 = scene.ports.get(&Address {
            instance: "sum".to_owned(),
            port: 2,
        });
        let port_rows: Vec<&RowWrite> = delta
            .rows
            .iter()
            .filter(|w| w.table == Table::Port)
            .collect();
        assert_eq!(
            port_rows,
            [&RowWrite {
                table: Table::Port,
                slot: sum2.copied().unwrap_or(u32::MAX),
            }],
            "only sum@2 writes a port row"
        );
    }
}
