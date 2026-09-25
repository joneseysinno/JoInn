//! ∂(body): every port no internal wire consumes, with direction and frame.

use joinn_dna::{Body, Cell, Direction, GenomeTarget};
use joinn_frame::{FrameRef, Hash, Verdict};
use joinn_prim::prim_ports;
use std::collections::{BTreeMap, BTreeSet};

use crate::Address;

/// One port of ∂(body). Address is identity; direction and frame come from the contract.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BoundaryPort {
    /// Instance and position.
    pub address: Address,
    /// In or out, from the cell contract or the primitive port table.
    pub direction: Direction,
    /// Frame, including version.
    pub frame: FrameRef,
}

/// Every port of every instance that no wire consumes.
/// A genome entry whose cell is not supplied is refused, never skipped.
pub fn membrane(body: &Body, cells: &BTreeMap<Hash, Cell>) -> Verdict<BTreeSet<BoundaryPort>> {
    let mut consumed = BTreeSet::new();
    for wire in &body.coding.wires {
        consumed.insert((wire.src_instance.clone(), wire.src_port));
        consumed.insert((wire.dst_instance.clone(), wire.dst_port));
    }
    let mut set = BTreeSet::new();
    for entry in &body.coding.genome {
        let ports = match &entry.target {
            GenomeTarget::Cell(hash) => match cells.get(hash) {
                Some(cell) => cell.coding.contract.ports.clone(),
                None => {
                    let names = if entry.instances.is_empty() {
                        "(none)".to_owned()
                    } else {
                        entry.instances.join(", ")
                    };
                    return crate::refuse(format!(
                        "instance {names} cell {} was not supplied; acceptance is that cell in the cell map",
                        hash.short_hex()
                    ));
                }
            },
            GenomeTarget::Prim(name) => match prim_ports(name) {
                Some(ports) => ports,
                None => {
                    return crate::refuse(format!(
                        "primitive {name} has no port table; acceptance is a floor primitive"
                    ));
                }
            },
        };
        for instance in &entry.instances {
            for port in &ports {
                if consumed.contains(&(instance.clone(), port.position)) {
                    continue;
                }
                set.insert(BoundaryPort {
                    address: Address {
                        instance: instance.clone(),
                        port: port.position,
                    },
                    direction: port.direction,
                    frame: port.frame,
                });
            }
        }
    }
    Verdict::Ok(set)
}

#[cfg(test)]
mod tests {
    use joinn_dna::{Wire, format_cell, hash, parse_body, sum_cell};
    use joinn_frame::{FrameRegistry, Verdict};
    use std::collections::{BTreeMap, BTreeSet};

    use super::membrane;

    fn calculator() -> (
        joinn_dna::Body,
        BTreeMap<joinn_frame::Hash, joinn_dna::Cell>,
    ) {
        let format = format_cell();
        let fh = hash(&format.coding);
        let cli = joinn_dna::cli_input_cell(fh);
        let sum = sum_cell();
        let src = include_str!("../../../corpus/phase2/calculator.body");
        let body = match parse_body(src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mut cells = BTreeMap::new();
        cells.insert(fh, format);
        cells.insert(hash(&cli.coding), cli);
        cells.insert(hash(&sum.coding), sum);
        (body, cells)
    }

    fn printed(
        body: &joinn_dna::Body,
        cells: &BTreeMap<joinn_frame::Hash, joinn_dna::Cell>,
    ) -> BTreeSet<String> {
        match membrane(body, cells) {
            Verdict::Ok(set) => set.iter().map(|a| a.address.printed()).collect(),
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn calculator_membrane_is_three_ports() {
        let (body, cells) = calculator();
        let set = printed(&body, &cells);
        assert_eq!(
            set,
            BTreeSet::from([
                "cli_a@0".to_string(),
                "cli_b@0".to_string(),
                "sum@2".to_string(),
            ])
        );
    }

    #[test]
    fn an_extra_wire_makes_a_strictly_smaller_membrane() {
        let (mut body, cells) = calculator();
        let before = printed(&body, &cells);
        body.coding.wires.push(Wire {
            src_instance: "sum".into(),
            src_port: 2,
            dst_instance: "cli_a".into(),
            dst_port: 0,
        });
        let after = printed(&body, &cells);
        assert!(after.len() < before.len());
        assert!(after.is_subset(&before));
        assert_ne!(after, before);
    }
}
