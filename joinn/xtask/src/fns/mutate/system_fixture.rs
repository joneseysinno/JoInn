//! Test fixture: a hand-written counting system and its growing contact.

use joinn_dna::{
    Accept, ForceKind, System, SystemBody, SystemCoding, SystemForce, SystemRegulatory, hash,
};
use joinn_frame::{FrameId, FrameRef, Hash};
use std::collections::BTreeMap;

use crate::fns::subject::{Subject, SystemSubject};

use super::growing_contact::growing_contact;

const SUM: &str = "6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39";

/// Counting: one body bound as `numbers`, `combine ℤ 1 … as count on numbers`.
pub(crate) fn system_fixture() -> Subject {
    let contact = growing_contact(Accept::One, None);
    let system = System {
        coding: SystemCoding {
            codex: 1,
            bodies: vec![SystemBody {
                contact: hash(&contact.coding),
                alias: "numbers".to_owned(),
            }],
            forces: vec![SystemForce {
                kind: ForceKind::Combine,
                frame: FrameRef::new(FrameId::Int, 1),
                response: Hash::parse_hex(SUM).unwrap_or_else(|| panic!("hex {SUM}")),
                name: "count".to_owned(),
                on: "numbers".to_owned(),
            }],
            lineage: None,
        },
        regulatory: SystemRegulatory {
            names: BTreeMap::from([("count".to_owned(), "Count".to_owned())]),
            present: BTreeMap::from([("count".to_owned(), "{0}".to_owned())]),
            waiting: BTreeMap::from([("numbers".to_owned(), 1)]),
        },
    };
    Subject::System(SystemSubject {
        system,
        contacts: BTreeMap::from([("numbers".to_owned(), contact)]),
    })
}
