//! A link is admitted when its members agree: direction and frame.

use joinn_dna::{Direction, GenomeTarget};
use joinn_frame::{FrameRef, Verdict};
use joinn_prim::prim_ports;
use std::collections::BTreeMap;

use crate::bind::Bound;
use crate::universe::{Mark, Universe};

type PortIndex = BTreeMap<(String, u32), (Direction, FrameRef)>;

/// Tails are out-ports, heads are in-ports, and every member's frame is equal.
pub fn check_link_types(universe: &Universe, bound: &Bound) -> Verdict<()> {
    let mut ports: BTreeMap<String, PortIndex> = BTreeMap::new();
    for (alias, (body, cells)) in bound.iter() {
        let mut index = BTreeMap::new();
        for entry in &body.coding.genome {
            let decls = match &entry.target {
                GenomeTarget::Cell(hash) => match cells.get(hash) {
                    Some(cell) => cell.coding.contract.ports.clone(),
                    None => {
                        return crate::refuse(format!(
                            "alias {alias} instance {} cell {} was not supplied",
                            entry.instances.join(", "),
                            hash.short_hex()
                        ));
                    }
                },
                GenomeTarget::Prim(name) => match prim_ports(name) {
                    Some(p) => p,
                    None => {
                        return crate::refuse(format!("primitive {name} has no port table"));
                    }
                },
            };
            for instance in &entry.instances {
                for port in &decls {
                    index.insert(
                        (instance.clone(), port.position),
                        (port.direction, port.frame),
                    );
                }
            }
        }
        ports.insert(alias.to_owned(), index);
    }

    for link in &universe.coding.links {
        let mut required: Option<FrameRef> = None;
        for member in &link.members {
            let key = format!("{}.{}@{}", member.body, member.instance, member.port);
            let Some(index) = ports.get(&member.body) else {
                return crate::refuse(format!(
                    "link {} member {key} has no bound body; acceptance is a body for {}",
                    link.id, member.body
                ));
            };
            let Some((direction, frame)) = index.get(&(member.instance.clone(), member.port))
            else {
                return crate::refuse(format!(
                    "link {} member {key} no such port; acceptance is a declared port",
                    link.id
                ));
            };
            match member.mark {
                Mark::Tail if *direction != Direction::Out => {
                    return crate::refuse(format!(
                        "link {} member {key} is tail but direction is {direction:?}; acceptance is Out",
                        link.id
                    ));
                }
                Mark::Head if *direction != Direction::In => {
                    return crate::refuse(format!(
                        "link {} member {key} is head but direction is {direction:?}; acceptance is In",
                        link.id
                    ));
                }
                _ => {}
            }
            match required {
                None => required = Some(*frame),
                Some(need) if need != *frame => {
                    return crate::refuse(format!(
                        "link {} member {key} is {frame} beside {need}; acceptance is one frame for every member",
                        link.id
                    ));
                }
                Some(_) => {}
            }
        }
    }
    Verdict::Ok(())
}
