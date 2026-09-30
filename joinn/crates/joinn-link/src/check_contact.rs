//! Admit a contact body: its cells, grants, members, receptors, pins and arity.

use joinn_dna::{
    Body, BodyCoding, Cell, Contact, Direction, GenomeEntry, GenomeTarget, PortDecl, check_body,
};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_prim::forces::response;
use std::collections::{BTreeMap, BTreeSet};

/// The first refusal, in this order:
/// 1. every genome cell is supplied (`check_body`'s refusal);
/// 2. every granted instance exists;
/// 3. every member names an instance and a port it has;
/// 4. every member's port is in the force's frame (receptor);
/// 5. every member is an out-port;
/// 6. no port is reached by two forces;
/// 7. the pinned response is the register's;
/// 8. the member count is the response's in-port count, which needs
/// 9. the response cell supplied.
///
/// A member may name a response (a chained force). Order-freedom is not
/// resampled: the register was admitted by `check_register`.
pub fn check_contact(
    contact: &Contact,
    cells: &BTreeMap<Hash, Cell>,
    _frames: &FrameRegistry,
) -> Verdict<()> {
    let coding = &contact.coding;
    let genome_view = Body {
        coding: BodyCoding {
            codex: coding.codex,
            declarations: Vec::new(),
            genome: coding
                .genome
                .iter()
                .map(|g| GenomeEntry {
                    target: GenomeTarget::Cell(g.cell),
                    instances: g.instances.clone(),
                })
                .collect(),
            grants: BTreeMap::new(),
            reads: BTreeSet::new(),
            wires: Vec::new(),
            budget_steps: coding.budget_steps,
            lineage: coding.lineage,
        },
        regulatory: contact.regulatory.clone(),
    };
    if let Verdict::Refused(r) = check_body(&genome_view, cells) {
        return Verdict::Refused(r);
    }

    let missing_response = |force: &joinn_dna::Force| {
        crate::refuse(format!(
            "force {}'s response cell:{} was not supplied; acceptance is that cell in the cell map",
            force.name, force.response
        ))
    };
    let mut ports: BTreeMap<&str, &[PortDecl]> = BTreeMap::new();
    for entry in &coding.genome {
        if let Some(cell) = cells.get(&entry.cell) {
            for instance in &entry.instances {
                ports.insert(instance.as_str(), &cell.coding.contract.ports);
            }
        }
    }
    let mut responses: BTreeMap<&str, Option<&Cell>> = BTreeMap::new();
    for force in &coding.forces {
        let cell = cells.get(&force.response);
        if let Some(cell) = cell {
            ports.insert(force.name.as_str(), &cell.coding.contract.ports);
        }
        responses.insert(force.name.as_str(), cell);
    }

    for (cap, instances) in &coding.grants {
        for instance in instances {
            if !ports.contains_key(instance.as_str()) && !responses.contains_key(instance.as_str())
            {
                return crate::refuse(format!(
                    "grant {cap} names {instance}, which is no instance; acceptance is an instance of the genome or a force's response"
                ));
            }
        }
    }

    let mut reached: BTreeMap<(&str, u32), &str> = BTreeMap::new();
    for force in &coding.forces {
        let frame = force.frame;
        for member in &force.members {
            let Some(decls) = ports.get(member.instance.as_str()) else {
                if let Some(None) = responses.get(member.instance.as_str()) {
                    return missing_response(force);
                }
                return crate::refuse(format!(
                    "member {member} names no instance; acceptance is an instance of the genome or a force's response"
                ));
            };
            let Some(port) = decls.iter().find(|p| p.position == member.port) else {
                return crate::refuse(format!(
                    "member {member} names no port; acceptance is a port of {}",
                    member.instance
                ));
            };
            if port.frame != frame {
                return crate::refuse(format!(
                    "receptor: {member} is {}, the force is {} {frame}; acceptance is a member in {frame}",
                    port.frame,
                    force.kind.word()
                ));
            }
            if port.direction != Direction::Out {
                return crate::refuse(format!(
                    "member {member} is an in-port; acceptance is an out-port"
                ));
            }
            if let Some(other) =
                reached.insert((member.instance.as_str(), member.port), force.name.as_str())
            {
                return crate::refuse(format!(
                    "{member} is reached by forces {other} and {}; acceptance is one force per port (R85)",
                    force.name
                ));
            }
        }
    }

    for force in &coding.forces {
        let word = force.kind.word();
        let frame = force.frame;
        match response(force.kind, &frame) {
            None => {
                return crate::refuse(format!(
                    "{word} on {frame} has no registered response; acceptance is a frame in the force register"
                ));
            }
            Some(registered) if registered != force.response => {
                return crate::refuse(format!(
                    "{word} on {frame} responds by cell:{registered}; force {} pins cell:{}. acceptance is the registered response",
                    force.name, force.response
                ));
            }
            Some(_) => {}
        }
        let Some(Some(cell)) = responses.get(force.name.as_str()) else {
            return missing_response(force);
        };
        let k = cell
            .coding
            .contract
            .ports
            .iter()
            .filter(|p| p.direction == Direction::In)
            .count();
        let n = force.members.len();
        if n != k {
            return crate::refuse(format!(
                "force {} has {n} members; its response takes {k}. acceptance is {k} members (R84)",
                force.name
            ));
        }
    }
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use joinn_dna::{Cell, hash, parse_cell, parse_contact};
    use joinn_frame::{FrameRegistry, Hash, Verdict};
    use std::collections::BTreeMap;

    use super::check_contact;

    const CALCULATOR: &str = include_str!("../../../corpus/phase7/calculator.contact");
    const SUM: &str = "6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39";
    const CLI: &str = "c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e";
    const MUL: &str = "12b6e5458891b14aa74e43cee34b1184be04547fe58e035b1882125467120fa7";
    const RAT_SUM: &str = "fc5403ff7ee2fbe0c843d40998a74411caef19479a17d5d9376cd9ea8c2c8b73";

    fn cells() -> BTreeMap<Hash, Cell> {
        let mut out = BTreeMap::new();
        for src in [
            include_str!("../../../corpus/phase0/sum.cell"),
            include_str!("../../../corpus/phase0/cli_input.cell"),
            include_str!("../../../corpus/phase0/format.cell"),
            include_str!("../../../corpus/phase21/mul.cell"),
            include_str!("../../../corpus/phase21/rat_sum.cell"),
        ] {
            match parse_cell(src, &FrameRegistry::phase1()) {
                Verdict::Ok(c) => {
                    out.insert(hash(&c.coding), c);
                }
                Verdict::Refused(r) => panic!("{}", r.reason),
            }
        }
        out
    }

    fn check(src: &str, cells: &BTreeMap<Hash, Cell>) -> Verdict<()> {
        let frames = FrameRegistry::phase1();
        match parse_contact(src, &frames) {
            Verdict::Ok(c) => check_contact(&c, cells, &frames),
            Verdict::Refused(r) => panic!("parse: {}", r.reason),
        }
    }

    fn refused(src: &str, cells: &BTreeMap<Hash, Cell>) -> String {
        match check(src, cells) {
            Verdict::Refused(r) => r.reason,
            Verdict::Ok(()) => panic!("admitted:\n{src}"),
        }
    }

    fn forces(lines: &str) -> String {
        CALCULATOR.replace(
            &format!("    combine ℤ 1 cell:{SUM} as sum from cli_a@1, cli_b@1\n"),
            lines,
        )
    }

    #[test]
    fn the_calculator_is_admitted() {
        assert!(matches!(check(CALCULATOR, &cells()), Verdict::Ok(())));
    }

    #[test]
    fn a_missing_genome_cell_is_refused_as_check_body_refuses_it() {
        let mut cells = cells();
        cells.retain(|h, _| h.to_hex() != CLI);
        assert_eq!(
            refused(CALCULATOR, &cells),
            format!("unknown cell hash {CLI}")
        );
    }

    #[test]
    fn a_grant_to_no_instance_is_refused() {
        let src = CALCULATOR.replace("stdin: cli_a, cli_b", "stdin: cli_a, cli_b, ghost");
        assert_eq!(
            refused(&src, &cells()),
            "grant stdin names ghost, which is no instance; acceptance is an instance of the genome or a force's response"
        );
    }

    #[test]
    fn a_member_of_no_instance_or_no_port_is_refused() {
        let src = CALCULATOR.replace("from cli_a@1, cli_b@1", "from cli_a@1, ghost@1");
        assert_eq!(
            refused(&src, &cells()),
            "member ghost@1 names no instance; acceptance is an instance of the genome or a force's response"
        );
        let src = CALCULATOR.replace("from cli_a@1, cli_b@1", "from cli_a@7, cli_b@1");
        assert_eq!(
            refused(&src, &cells()),
            "member cli_a@7 names no port; acceptance is a port of cli_a"
        );
    }

    #[test]
    fn a_member_in_another_frame_is_refused_at_the_receptor_before_direction() {
        let src = CALCULATOR.replace("from cli_a@1, cli_b@1", "from cli_a@0, cli_b@1");
        assert_eq!(
            refused(&src, &cells()),
            "receptor: cli_a@0 is Text 1, the force is combine ℤ 1; acceptance is a member in ℤ 1"
        );
    }

    #[test]
    fn an_in_port_member_is_refused() {
        let src = forces(&format!(
            "    combine ℤ 1 cell:{SUM} as sum from cli_a@1, cli_b@1\n    combine ℤ 1 cell:{SUM} as total from sum@0, sum@2\n"
        ));
        assert_eq!(
            refused(&src, &cells()),
            "member sum@0 is an in-port; acceptance is an out-port"
        );
    }

    #[test]
    fn a_port_reached_by_two_forces_is_refused() {
        let src = forces(&format!(
            "    combine ℤ 1 cell:{SUM} as s1 from cli_a@1, cli_b@1\n    combine ℤ 1 cell:{SUM} as s2 from cli_a@1, s1@2\n"
        ));
        assert_eq!(
            refused(&src, &cells()),
            "cli_a@1 is reached by forces s1 and s2; acceptance is one force per port (R85)"
        );
    }

    #[test]
    fn a_pin_the_register_does_not_hold_is_refused() {
        let src = CALCULATOR.replace(&format!("cell:{SUM} as sum"), &format!("cell:{MUL} as sum"));
        assert_eq!(
            refused(&src, &cells()),
            format!(
                "combine on ℤ 1 responds by cell:{SUM}; force sum pins cell:{MUL}. acceptance is the registered response"
            )
        );
        let rat = format!(
            "contact {{ codex 1 genome {{ cell:{RAT_SUM} as r1, r2 }} forces {{ combine ℚ 1 cell:{RAT_SUM} as s from r1@2, r2@2 }} budget {{ steps 100 }} lineage none }}\n"
        );
        assert_eq!(
            refused(&rat, &cells()),
            "combine on ℚ 1 has no registered response; acceptance is a frame in the force register"
        );
    }

    #[test]
    fn a_member_count_other_than_the_responses_arity_is_refused() {
        let src = CALCULATOR.replace("from cli_a@1, cli_b@1", "from cli_a@1");
        assert_eq!(
            refused(&src, &cells()),
            "force sum has 1 members; its response takes 2. acceptance is 2 members (R84)"
        );
    }

    #[test]
    fn a_response_cell_not_supplied_is_refused() {
        let mut cells = cells();
        cells.retain(|h, _| h.to_hex() != SUM);
        assert_eq!(
            refused(CALCULATOR, &cells),
            format!(
                "force sum's response cell:{SUM} was not supplied; acceptance is that cell in the cell map"
            )
        );
    }
}
