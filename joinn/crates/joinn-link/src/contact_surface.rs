//! The surface of a contact body: every port no force reaches, plus each
//! response's out-port.

use joinn_dna::{Cell, Contact, Direction, PortDecl};
use joinn_frame::{Hash, Verdict};
use std::collections::{BTreeMap, BTreeSet};

use crate::{Address, BoundaryPort};

/// Every port of every genome cell that is not a member of a force, and every
/// response port not fed by a member: its in-ports `0 … n−1` are fed by its
/// `n` members, and its out-port is on the surface unless another force
/// reaches it. Computed on demand, stored nowhere. A genome or response cell
/// that is not supplied is refused, never skipped.
pub fn contact_surface(
    contact: &Contact,
    cells: &BTreeMap<Hash, Cell>,
) -> Verdict<BTreeSet<BoundaryPort>> {
    let members: BTreeSet<(&str, u32)> = contact
        .coding
        .forces
        .iter()
        .flat_map(|f| f.members.iter().map(|m| (m.instance.as_str(), m.port)))
        .collect();
    let mut set = BTreeSet::new();
    let mut add = |instance: &str, port: &PortDecl| {
        set.insert(BoundaryPort {
            address: Address {
                instance: instance.to_owned(),
                port: port.position,
            },
            direction: port.direction,
            frame: port.frame,
        });
    };
    for entry in &contact.coding.genome {
        let Some(cell) = cells.get(&entry.cell) else {
            return crate::refuse(format!(
                "instance {} cell {} was not supplied; acceptance is that cell in the cell map",
                entry.instances.join(", "),
                entry.cell.short_hex()
            ));
        };
        for instance in &entry.instances {
            for port in &cell.coding.contract.ports {
                if !members.contains(&(instance.as_str(), port.position)) {
                    add(instance, port);
                }
            }
        }
    }
    for force in &contact.coding.forces {
        let Some(cell) = cells.get(&force.response) else {
            return crate::refuse(format!(
                "response {} cell {} was not supplied; acceptance is that cell in the cell map",
                force.name,
                force.response.short_hex()
            ));
        };
        let fed = force.members.len();
        for port in &cell.coding.contract.ports {
            let on_surface = match port.direction {
                Direction::In => port.position as usize >= fed,
                Direction::Out => !members.contains(&(force.name.as_str(), port.position)),
            };
            if on_surface {
                add(&force.name, port);
            }
        }
    }
    Verdict::Ok(set)
}
