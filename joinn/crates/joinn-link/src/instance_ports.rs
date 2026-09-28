//! Every instance's ports, from its cell's contract or the primitive port table.

use joinn_dna::{Body, Cell, GenomeTarget, PortDecl};
use joinn_frame::{Hash, Verdict};
use joinn_prim::prim_ports;
use std::collections::BTreeMap;

/// Every instance mapped to its ports in position order.
/// A genome entry whose cell is not supplied is refused, never skipped.
pub fn instance_ports(
    body: &Body,
    cells: &BTreeMap<Hash, Cell>,
) -> Verdict<BTreeMap<String, Vec<PortDecl>>> {
    let mut map = BTreeMap::new();
    for entry in &body.coding.genome {
        let mut ports = match &entry.target {
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
        ports.sort_by_key(|p| p.position);
        for instance in &entry.instances {
            map.insert(instance.clone(), ports.clone());
        }
    }
    Verdict::Ok(map)
}

#[cfg(test)]
mod tests {
    use joinn_dna::{Direction, format_cell, hash, parse_body, sum_cell};
    use joinn_frame::{FrameRegistry, Verdict};
    use std::collections::BTreeMap;

    use super::instance_ports;

    #[test]
    fn calculator_instances_have_their_contract_ports_in_position_order() {
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
        cells.insert(hash(&cli.coding), cli);
        cells.insert(hash(&sum.coding), sum);
        let map = match instance_ports(&body, &cells) {
            Verdict::Ok(m) => m,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let printed: Vec<String> = map
            .iter()
            .flat_map(|(i, ports)| {
                ports.iter().map(move |p| {
                    let d = match p.direction {
                        Direction::In => "in",
                        Direction::Out => "out",
                    };
                    format!("{i}@{} {d}", p.position)
                })
            })
            .collect();
        assert_eq!(
            printed,
            [
                "cli_a@0 in",
                "cli_a@1 out",
                "cli_b@0 in",
                "cli_b@1 out",
                "sum@0 in",
                "sum@1 in",
                "sum@2 out",
            ]
        );

        cells.retain(|_, c| c.coding.contract.ports.len() == 3);
        match instance_ports(&body, &cells) {
            Verdict::Refused(r) => assert!(
                r.reason.starts_with("instance cli_a, cli_b cell ")
                    && r.reason
                        .ends_with(" was not supplied; acceptance is that cell in the cell map"),
                "{}",
                r.reason
            ),
            Verdict::Ok(_) => panic!("a missing cell must be refused, never skipped"),
        }
    }
}
