//! Every cell's role in a contact body, from its surface and its forces.

use joinn_dna::{Cell, Contact};
use joinn_frame::{Hash, Verdict};
use std::collections::{BTreeMap, BTreeSet};

use super::{BodyRole, Facing, Holding};
use crate::contact_surface::contact_surface;

/// Instance name → role, for every genome instance and every force's
/// response. A cell faces out when any of its ports is on `contact_surface`,
/// and reacts when it is a force's response. Admission is `check_contact`'s;
/// a cell that is not supplied is refused, as the surface refuses it.
pub fn body_roles(
    contact: &Contact,
    cells: &BTreeMap<Hash, Cell>,
) -> Verdict<BTreeMap<String, BodyRole>> {
    let surface = match contact_surface(contact, cells) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let facing_out: BTreeSet<&str> = surface
        .iter()
        .map(|p| p.address.instance.as_str())
        .collect();
    let facing = |name: &str| {
        if facing_out.contains(name) {
            Facing::Out
        } else {
            Facing::In
        }
    };
    let mut roles = BTreeMap::new();
    for entry in &contact.coding.genome {
        for instance in &entry.instances {
            roles.insert(
                instance.clone(),
                BodyRole {
                    facing: facing(instance),
                    holding: Holding::Holds,
                },
            );
        }
    }
    for force in &contact.coding.forces {
        roles.insert(
            force.name.clone(),
            BodyRole {
                facing: facing(&force.name),
                holding: Holding::Reacts,
            },
        );
    }
    Verdict::Ok(roles)
}

#[cfg(test)]
mod tests {
    use super::body_roles;
    use crate::roles::BodyRole;
    use joinn_dna::{Cell, Contact, Direction, hash, parse_cell, parse_contact};
    use joinn_frame::{FrameRegistry, Hash, Verdict};
    use std::collections::BTreeMap;

    const SUM: &str = "6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39";
    const CLI: &str = "c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e";

    fn cells() -> BTreeMap<Hash, Cell> {
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
        cells
    }

    fn contact(src: &str) -> Contact {
        match parse_contact(src, &FrameRegistry::phase1()) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    fn printed(roles: &BTreeMap<String, BodyRole>) -> Vec<String> {
        roles
            .iter()
            .map(|(name, role)| {
                format!(
                    "{name} {} ({}, {})",
                    role.word(),
                    role.facing.word(),
                    role.holding.word()
                )
            })
            .collect()
    }

    fn roles_of(c: &Contact, cells: &BTreeMap<Hash, Cell>) -> BTreeMap<String, BodyRole> {
        match body_roles(c, cells) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn the_calculator_is_two_protects_and_a_carry() {
        let c = contact(include_str!("../../../../corpus/phase7/calculator.contact"));
        assert_eq!(
            printed(&roles_of(&c, &cells())),
            [
                "cli_a protect (faces out, holds)",
                "cli_b protect (faces out, holds)",
                "sum carry (faces out, reacts)",
            ]
        );
    }

    #[test]
    fn roles_do_not_move_under_the_neutral_edit_or_a_regulatory_rewrite() {
        let cells = cells();
        let c = contact(include_str!("../../../../corpus/phase7/calculator.contact"));
        let before = roles_of(&c, &cells);
        let mut neutral = c.clone();
        for label in neutral.regulatory.labels.values_mut() {
            label.push_str(" (neutral)");
        }
        assert_ne!(neutral.regulatory, c.regulatory);
        assert_eq!(roles_of(&neutral, &cells), before);
        let mut rewritten = c.clone();
        rewritten.regulatory = Default::default();
        assert_ne!(rewritten.regulatory, c.regulatory);
        assert_eq!(roles_of(&rewritten, &cells), before);
    }

    #[test]
    fn a_response_that_a_force_reaches_responds() {
        let c = contact(&format!(
            "contact {{ codex 1 genome {{ cell:{CLI} as a, b, c }} grants {{ stdin: a, b, c }} forces {{ combine Z 1 cell:{SUM} as s1 from a@1, b@1 combine Z 1 cell:{SUM} as s2 from c@1, s1@2 }} budget {{ steps 100000 }} lineage none }}"
        ));
        assert_eq!(
            printed(&roles_of(&c, &cells())),
            [
                "a protect (faces out, holds)",
                "b protect (faces out, holds)",
                "c protect (faces out, holds)",
                "s1 respond (faces in, reacts)",
                "s2 carry (faces out, reacts)",
            ]
        );
    }

    #[test]
    fn a_cell_whose_only_port_a_force_reaches_stores() {
        let mut cells = cells();
        let Some(sum) = Hash::parse_hex(SUM).and_then(|h| cells.get(&h)).cloned() else {
            panic!("sum cell");
        };
        let mut only_out = sum.clone();
        let Some(mut port) = only_out
            .coding
            .contract
            .ports
            .iter()
            .find(|p| p.direction == Direction::Out)
            .cloned()
        else {
            panic!("sum has an out-port");
        };
        port.position = 0;
        only_out.coding.contract.ports = vec![port];
        let only_out_hash = hash(&only_out.coding);
        cells.insert(only_out_hash, only_out);
        let c = contact(&format!(
            "contact {{ codex 1 genome {{ cell:{} as k cell:{CLI} as a }} grants {{ stdin: a }} forces {{ combine Z 1 cell:{SUM} as s from a@1, k@0 }} budget {{ steps 100000 }} lineage none }}",
            only_out_hash.to_hex()
        ));
        assert_eq!(
            printed(&roles_of(&c, &cells)),
            [
                "a protect (faces out, holds)",
                "k store (faces in, holds)",
                "s carry (faces out, reacts)",
            ]
        );
    }

    #[test]
    fn a_missing_cell_is_refused() {
        let c = contact(include_str!("../../../../corpus/phase7/calculator.contact"));
        let Verdict::Refused(r) = body_roles(&c, &BTreeMap::new()) else {
            panic!("no cells: must be refused");
        };
        assert!(r.reason.contains("was not supplied"), "{}", r.reason);
    }
}
