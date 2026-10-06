//! A system with the corpus contacts it binds, by alias.

use joinn_dna::{System, hash};
use joinn_frame::Verdict;
use std::collections::BTreeMap;

use super::layout::corpus_contacts;
use super::subject::SystemSubject;

/// Each body's contact is the corpus `.contact` whose coding hash it binds.
pub(crate) fn system_subject(system: System) -> Result<SystemSubject, String> {
    let (parsed, _) = corpus_contacts()?;
    let by_hash: BTreeMap<_, _> = parsed
        .into_iter()
        .filter_map(|(_, v)| match v {
            Verdict::Ok(c) => Some((hash(&c.coding), c)),
            Verdict::Refused(_) => None,
        })
        .collect();
    let mut contacts = BTreeMap::new();
    for body in &system.coding.bodies {
        let Some(contact) = by_hash.get(&body.contact) else {
            return Err(format!(
                "system: body {} binds contact:{}, which no corpus contact hashes to; acceptance is a corpus contact",
                body.alias, body.contact
            ));
        };
        contacts.insert(body.alias.clone(), contact.clone());
    }
    Ok(SystemSubject { system, contacts })
}
