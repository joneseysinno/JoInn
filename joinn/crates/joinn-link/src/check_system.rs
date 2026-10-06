//! Admit a system: its growing bodies, its forces, and its lineage.

use joinn_dna::{Cell, Contact, Direction, System, hash};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_prim::forces::response;
use std::collections::BTreeMap;

use crate::check_contact::check_contact;

/// The first refusal, in this order:
/// 1. every bound contact is supplied and passes `check_contact`;
/// 2. every bound contact grows;
/// 3. there is a force;
/// 4. every force is on a bound alias;
/// 5. receptor: the grown cell has one out-port, in the force's frame;
/// 6. the pinned response is the register's, and its cell is supplied;
/// 7. a named parent is supplied (in `systems`, its contacts in `contacts`),
///    and each of its bodies grows the cell the child's body of that alias
///    grows.
pub fn check_system(
    system: &System,
    contacts: &BTreeMap<Hash, Contact>,
    systems: &BTreeMap<Hash, System>,
    cells: &BTreeMap<Hash, Cell>,
    frames: &FrameRegistry,
) -> Verdict<()> {
    let coding = &system.coding;
    let mut grown: BTreeMap<&str, Hash> = BTreeMap::new();
    for body in &coding.bodies {
        let Some(contact) = contacts.get(&body.contact) else {
            return crate::refuse(format!(
                "system: body {} binds contact:{}, which was not supplied; acceptance is that contact",
                body.alias, body.contact
            ));
        };
        if let Verdict::Refused(r) = check_contact(contact, cells, frames) {
            return Verdict::Refused(r);
        }
        let Some(grows) = &contact.coding.grows else {
            return crate::refuse(format!(
                "system: body {} does not grow; acceptance is a contact with a grows section (Phase 7.4)",
                body.alias
            ));
        };
        grown.insert(body.alias.as_str(), grows.cell);
    }
    if coding.forces.is_empty() {
        return crate::refuse(
            "system: no force; acceptance is a force (a system is the bodies its forces reach)",
        );
    }
    for force in &coding.forces {
        let Some(cell_hash) = grown.get(force.on.as_str()) else {
            return crate::refuse(format!(
                "system: force {} is on {}, which is no body; acceptance is a bound alias",
                force.name, force.on
            ));
        };
        let Some(cell) = cells.get(cell_hash) else {
            return crate::refuse(format!(
                "system: {} grows cell:{cell_hash}, which was not supplied; acceptance is that cell in the cell map",
                force.on
            ));
        };
        let outs: Vec<_> = cell
            .coding
            .contract
            .ports
            .iter()
            .filter(|p| p.direction == Direction::Out)
            .collect();
        let frame = force.frame;
        if outs.len() != 1 || outs[0].frame != frame {
            let got: Vec<String> = outs.iter().map(|p| format!("{}", p.frame)).collect();
            return crate::refuse(format!(
                "receptor: {} grows out-ports [{}], the force is {} {frame}; acceptance is one out-port in {frame}",
                force.on,
                got.join(", "),
                force.kind.word()
            ));
        }
        let word = force.kind.word();
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
        if !cells.contains_key(&force.response) {
            return crate::refuse(format!(
                "force {}'s response cell:{} was not supplied; acceptance is that cell in the cell map",
                force.name, force.response
            ));
        }
    }
    let Some(parent_hash) = coding.lineage else {
        return Verdict::Ok(());
    };
    let Some(parent) = systems.get(&parent_hash) else {
        return crate::refuse(format!(
            "system: lineage {parent_hash} was not supplied; acceptance is the parent system"
        ));
    };
    if hash(&parent.coding) != parent_hash {
        return crate::refuse(format!(
            "system: the system supplied for lineage {parent_hash} hashes to {}; acceptance is the parent itself",
            hash(&parent.coding)
        ));
    }
    for body in &parent.coding.bodies {
        let parent_cell = contacts
            .get(&body.contact)
            .and_then(|c| c.coding.grows.as_ref())
            .map(|g| g.cell);
        let Some(parent_cell) = parent_cell else {
            return crate::refuse(format!(
                "system: parent body {} binds contact:{}, which was not supplied or does not grow; acceptance is the parent's growing contact",
                body.alias, body.contact
            ));
        };
        match grown.get(body.alias.as_str()) {
            Some(child_cell) if *child_cell == parent_cell => {}
            Some(child_cell) => {
                return crate::refuse(format!(
                    "evolution: {} grows cell:{child_cell} where its parent grows cell:{parent_cell}; acceptance is the same cell (evolution may change what a body accepts, not what it grows)",
                    body.alias
                ));
            }
            None => {
                return crate::refuse(format!(
                    "evolution: the parent's body {} is not in the child; acceptance is every parent body, by alias",
                    body.alias
                ));
            }
        }
    }
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use joinn_dna::{Cell, Contact, System, hash, parse_cell, parse_contact, parse_system};
    use joinn_frame::{FrameRegistry, Hash, Verdict};
    use std::collections::BTreeMap;

    use super::check_system;

    const SUM: &str = "6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39";
    const CLI: &str = "c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e";
    const MUL: &str = "12b6e5458891b14aa74e43cee34b1184be04547fe58e035b1882125467120fa7";

    fn cells() -> BTreeMap<Hash, Cell> {
        let mut out = BTreeMap::new();
        for src in [
            include_str!("../../../corpus/phase0/sum.cell"),
            include_str!("../../../corpus/phase0/cli_input.cell"),
            include_str!("../../../corpus/phase0/format.cell"),
            include_str!("../../../corpus/phase21/mul.cell"),
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

    fn contact(accepts: &str, grows: bool) -> Contact {
        let grows = if grows {
            format!("grows {{ cell:{CLI} as numbers accepts {accepts} }}")
        } else {
            format!("genome {{ cell:{CLI} as a }}")
        };
        let src = format!("contact {{ codex 1 {grows} budget {{ steps 100000 }} lineage none }}\n");
        match parse_contact(&src, &FrameRegistry::phase1()) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    fn system(contact: &Contact, forces: &str, lineage: &str) -> System {
        let src = format!(
            "system {{ codex 1 bodies {{ contact:{} as numbers }} forces {{ {forces} }} lineage {lineage} }}\n",
            hash(&contact.coding)
        );
        match parse_system(&src) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    fn count() -> String {
        format!("combine ℤ 1 cell:{SUM} as count on numbers")
    }

    fn check(
        s: &System,
        contacts: &[&Contact],
        systems: &[&System],
        cells: &BTreeMap<Hash, Cell>,
    ) -> Verdict<()> {
        let contacts = contacts
            .iter()
            .map(|c| (hash(&c.coding), (*c).clone()))
            .collect();
        let systems = systems
            .iter()
            .map(|s| (hash(&s.coding), (*s).clone()))
            .collect();
        check_system(s, &contacts, &systems, cells, &FrameRegistry::phase1())
    }

    fn refused(v: Verdict<()>) -> String {
        match v {
            Verdict::Refused(r) => r.reason,
            Verdict::Ok(()) => panic!("admitted"),
        }
    }

    #[test]
    fn counting_is_admitted() {
        let c = contact("one", true);
        let s = system(&c, &count(), "none");
        assert!(matches!(check(&s, &[&c], &[], &cells()), Verdict::Ok(())));
    }

    #[test]
    fn rule_1_a_contact_not_supplied_or_not_admitted_is_refused() {
        let c = contact("one", true);
        let s = system(&c, &count(), "none");
        assert_eq!(
            refused(check(&s, &[], &[], &cells())),
            format!(
                "system: body numbers binds contact:{}, which was not supplied; acceptance is that contact",
                hash(&c.coding)
            )
        );
        let plain = contact("one", false);
        let s = system(&plain, &count(), "none");
        let mut cells = cells();
        cells.retain(|h, _| h.to_hex() != CLI);
        assert_eq!(
            refused(check(&s, &[&plain], &[], &cells)),
            format!("unknown cell hash {CLI}")
        );
    }

    #[test]
    fn rule_2_a_body_that_does_not_grow_is_refused() {
        let plain = contact("one", false);
        let s = system(&plain, &count(), "none");
        assert_eq!(
            refused(check(&s, &[&plain], &[], &cells())),
            "system: body numbers does not grow; acceptance is a contact with a grows section (Phase 7.4)"
        );
    }

    #[test]
    fn rule_3_a_system_with_no_force_is_refused() {
        let c = contact("one", true);
        let s = system(&c, "", "none");
        assert_eq!(
            refused(check(&s, &[&c], &[], &cells())),
            "system: no force; acceptance is a force (a system is the bodies its forces reach)"
        );
    }

    #[test]
    fn rule_4_a_force_on_no_body_is_refused() {
        let c = contact("one", true);
        let s = system(&c, &count().replace("on numbers", "on ghost"), "none");
        assert_eq!(
            refused(check(&s, &[&c], &[], &cells())),
            "system: force count is on ghost, which is no body; acceptance is a bound alias"
        );
    }

    #[test]
    fn rule_5_a_force_in_another_frame_is_refused_at_the_receptor() {
        let c = contact("one", true);
        let s = system(&c, &count().replace("ℤ 1", "Text 1"), "none");
        assert_eq!(
            refused(check(&s, &[&c], &[], &cells())),
            "receptor: numbers grows out-ports [ℤ 1], the force is combine Text 1; acceptance is one out-port in Text 1"
        );
    }

    #[test]
    fn rule_6_a_pin_the_register_does_not_hold_is_refused() {
        let c = contact("one", true);
        let s = system(&c, &count().replace(SUM, MUL), "none");
        assert_eq!(
            refused(check(&s, &[&c], &[], &cells())),
            format!(
                "combine on ℤ 1 responds by cell:{SUM}; force count pins cell:{MUL}. acceptance is the registered response"
            )
        );
        let mut cells = cells();
        cells.retain(|h, _| h.to_hex() != SUM);
        let s = system(&c, &count(), "none");
        assert_eq!(
            refused(check(&s, &[&c], &[], &cells)),
            format!(
                "force count's response cell:{SUM} was not supplied; acceptance is that cell in the cell map"
            )
        );
    }

    #[test]
    fn rule_7_a_parent_not_supplied_is_refused_and_a_supplied_one_is_admitted() {
        let parent_contact = contact("one", true);
        let parent = system(&parent_contact, &count(), "none");
        let child_contact = contact("any", true);
        let lineage = hash(&parent.coding).to_hex();
        let child = system(&child_contact, &count(), &lineage);
        assert_eq!(
            refused(check(&child, &[&child_contact], &[], &cells())),
            format!("system: lineage {lineage} was not supplied; acceptance is the parent system")
        );
        let admitted = check(
            &child,
            &[&child_contact, &parent_contact],
            &[&parent],
            &cells(),
        );
        assert!(matches!(admitted, Verdict::Ok(())), "{admitted:?}");

        let mut other = contact("any", true);
        let sum = Hash::parse_hex(SUM).unwrap_or_else(|| panic!("hex"));
        if let Some(g) = other.coding.grows.as_mut() {
            g.cell = sum;
        }
        let child = system(&other, &count(), &lineage);
        assert_eq!(
            refused(check(
                &child,
                &[&other, &parent_contact],
                &[&parent],
                &cells()
            )),
            format!(
                "evolution: numbers grows cell:{SUM} where its parent grows cell:{CLI}; acceptance is the same cell (evolution may change what a body accepts, not what it grows)"
            )
        );
    }
}
