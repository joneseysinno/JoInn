//! The engine's own plumbing: the deliveries a contact body needs, derived.

use joinn_dna::{Body, BodyCoding, Cell, Contact, GenomeEntry, GenomeTarget, Wire};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use std::collections::BTreeMap;

use crate::check_contact::check_contact;

/// Admit `contact`, then derive the body the engine runs. Its inputs are the
/// contact, its cells and the force register, and nothing else. For each force,
/// its members in canonical (instance, port) order feed the response's in-ports
/// `0 … k−1`. The result is never written, described or drawn.
pub fn lower(
    contact: &Contact,
    cells: &BTreeMap<Hash, Cell>,
    frames: &FrameRegistry,
) -> Verdict<Body> {
    if let Verdict::Refused(r) = check_contact(contact, cells, frames) {
        return Verdict::Refused(r);
    }
    let coding = &contact.coding;
    let mut genome: Vec<GenomeEntry> = coding
        .genome
        .iter()
        .map(|g| GenomeEntry {
            target: GenomeTarget::Cell(g.cell),
            instances: g.instances.clone(),
        })
        .collect();
    let mut wires = Vec::new();
    for force in &coding.forces {
        genome.push(GenomeEntry {
            target: GenomeTarget::Cell(force.response),
            instances: vec![force.name.clone()],
        });
        let mut members = force.members.clone();
        members.sort();
        for (i, member) in (0u32..).zip(members) {
            wires.push(Wire {
                src_instance: member.instance,
                src_port: member.port,
                dst_instance: force.name.clone(),
                dst_port: i,
            });
        }
    }
    Verdict::Ok(Body {
        coding: BodyCoding {
            codex: coding.codex,
            declarations: Vec::new(),
            genome,
            grants: coding.grants.clone(),
            reads: coding.reads.clone(),
            wires,
            budget_steps: coding.budget_steps,
            lineage: coding.lineage,
        },
        regulatory: contact.regulatory.clone(),
    })
}

#[cfg(test)]
mod tests {
    use joinn_dna::{Cell, hash, parse_body, parse_cell, parse_contact, print_body};
    use joinn_frame::{FrameRegistry, Hash, Verdict};
    use std::collections::BTreeMap;

    use super::lower;

    #[test]
    fn the_calculator_lowers_to_the_wired_calculator_byte_for_byte() {
        let frames = FrameRegistry::phase1();
        let mut cells: BTreeMap<Hash, Cell> = BTreeMap::new();
        for src in [
            include_str!("../../../corpus/phase0/sum.cell"),
            include_str!("../../../corpus/phase0/cli_input.cell"),
        ] {
            match parse_cell(src, &frames) {
                Verdict::Ok(c) => {
                    cells.insert(hash(&c.coding), c);
                }
                Verdict::Refused(r) => panic!("{}", r.reason),
            }
        }
        let contact = match parse_contact(
            include_str!("../../../corpus/phase7/calculator.contact"),
            &frames,
        ) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let wired = match parse_body(
            include_str!("../../../corpus/phase2/calculator.body"),
            &frames,
        ) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let lowered = match lower(&contact, &cells, &frames) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(print_body(&lowered.coding), print_body(&wired.coding));
        assert_eq!(
            hash(&lowered.coding).to_hex(),
            "b55fba1eff65942099f6daf84b8bc47d605be05f637d805d260d3fcfd8c3ebde"
        );
        assert_eq!(lowered.regulatory, contact.regulatory);
    }
}
