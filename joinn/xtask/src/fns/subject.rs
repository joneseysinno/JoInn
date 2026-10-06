//! Parsed control-artifact subjects for non-legacy gates.

use joinn_dna::{Body, Contact, System};
use joinn_link::Universe;
use std::collections::BTreeMap;

use super::parse_lock_scores::LockRow;

/// A control artifact, parsed by file kind.
#[derive(Clone, Debug)]
pub(crate) enum Subject {
    Body(Body),
    Contact(Contact),
    System(SystemSubject),
    Universe(Universe),
    Lock(Vec<LockRow>),
    Transcript(Vec<String>),
    Text(String),
}

/// A system and the contacts it binds, by alias. A mutation of a bound
/// contact rebinds the system to the contact's new hash.
#[derive(Clone, Debug)]
pub(crate) struct SystemSubject {
    pub(crate) system: System,
    pub(crate) contacts: BTreeMap<String, Contact>,
}
