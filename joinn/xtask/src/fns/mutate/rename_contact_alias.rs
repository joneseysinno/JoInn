//! Rename an instance or a response everywhere it appears in a contact.

use joinn_dna::Contact;
use joinn_frame::Verdict;
use std::collections::BTreeMap;

use super::refuse::refuse;

pub(super) fn rename_contact_alias(contact: &mut Contact, from: &str, to: &str) -> Verdict<()> {
    let coding = &mut contact.coding;
    let names = |name: &str| {
        coding
            .genome
            .iter()
            .any(|g| g.instances.iter().any(|i| i == name))
            || coding.forces.iter().any(|f| f.name == name)
    };
    if !names(from) {
        return Verdict::Refused(refuse(format!(
            "no such instance or response {from}; acceptance is a name the contact declares"
        )));
    }
    if names(to) {
        return Verdict::Refused(refuse(format!(
            "{to} already exists; acceptance is a fresh name"
        )));
    }
    let rename = |name: &mut String| {
        if name == from {
            *name = to.to_owned();
        }
    };
    for entry in &mut coding.genome {
        entry.instances.iter_mut().for_each(rename);
    }
    for insts in coding.grants.values_mut() {
        insts.iter_mut().for_each(rename);
    }
    for force in &mut coding.forces {
        rename(&mut force.name);
        for member in &mut force.members {
            rename(&mut member.instance);
        }
    }
    let reg = &mut contact.regulatory;
    let rekey = |map: &mut BTreeMap<String, String>| {
        if let Some(v) = map.remove(from) {
            map.insert(to.to_owned(), v);
        }
    };
    rekey(&mut reg.prompts);
    rekey(&mut reg.present);
    rekey(&mut reg.names);
    rekey(&mut reg.labels);
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::contact_admission::contact_admission;
    use crate::fns::mutate::contact_fixture::contact_fixture;
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::subject::Subject;
    use joinn_dna::{hash, print_contact};
    use joinn_frame::Verdict;

    #[test]
    fn rename_alias_changes_only_that_name() {
        let s = contact_fixture();
        let Subject::Contact(orig) = &s else {
            panic!("contact");
        };
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::RenameAlias("sum", "total")) else {
            panic!("mutate");
        };
        let Subject::Contact(c) = &mutant else {
            panic!("contact mutant");
        };
        assert_ne!(hash(&c.coding), hash(&orig.coding));
        let was = print_contact(&orig.coding);
        let now = print_contact(&c.coding);
        let changed: Vec<(&str, &str)> = was
            .lines()
            .zip(now.lines())
            .filter(|(a, b)| a != b)
            .collect();
        assert_eq!(was.lines().count(), now.lines().count());
        assert_eq!(changed.len(), 1, "{changed:?}");
        assert_eq!(changed[0].0.replace(" as sum ", " as total "), changed[0].1);
        assert_eq!(
            c.regulatory.labels.get("total").map(String::as_str),
            Some("Sum")
        );
        assert!(c.regulatory.present.contains_key("total"));
        let admitted = contact_admission(&mutant);
        assert!(matches!(admitted, Verdict::Ok(())), "{admitted:?}");
        let Verdict::Refused(r) = mutate(&s, &Mutation::RenameAlias("sum", "cli_a")) else {
            panic!("a taken name must refuse");
        };
        assert!(r.reason.contains("cli_a"), "{}", r.reason);
    }
}
