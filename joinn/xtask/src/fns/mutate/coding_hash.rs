//! Coding hash of a body or universe subject, if it has one.

use crate::fns::subject::Subject;
use joinn_dna::hash;
use joinn_frame::Hash;
use joinn_link::hash_universe;

pub(super) fn coding_hash(s: &Subject) -> Option<Hash> {
    match s {
        Subject::Body(b) => Some(hash(&b.coding)),
        Subject::Universe(u) => Some(hash_universe(&u.coding)),
        Subject::Lock(_) | Subject::Transcript(_) | Subject::Text(_) => None,
    }
}
