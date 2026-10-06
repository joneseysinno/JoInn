//! Test fixture: a hand-written contact that grows the input cell.

use joinn_dna::{Accept, BodyRegulatory, Contact, ContactCoding, Grows};
use joinn_frame::Hash;
use std::collections::{BTreeMap, BTreeSet};

const CLI: &str = "c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e";

/// A contact with an empty genome that grows the input cell as `numbers`.
pub(crate) fn growing_contact(accepts: Accept, lineage: Option<Hash>) -> Contact {
    let cell = Hash::parse_hex(CLI).unwrap_or_else(|| panic!("hex {CLI}"));
    Contact {
        coding: ContactCoding {
            codex: 1,
            genome: Vec::new(),
            grants: BTreeMap::new(),
            reads: BTreeSet::new(),
            forces: Vec::new(),
            grows: Some(Grows {
                cell,
                name: "numbers".to_owned(),
                accepts,
            }),
            budget_steps: 100_000,
            lineage,
        },
        regulatory: BodyRegulatory::default(),
    }
}
