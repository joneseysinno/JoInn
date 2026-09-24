//! Parsed control-artifact subjects for non-legacy gates.

use joinn_dna::Body;
use joinn_link::Universe;

use super::parse_lock_scores::LockRow;

/// A control artifact, parsed by file kind.
#[derive(Clone, Debug)]
pub(crate) enum Subject {
    Body(Body),
    Universe(Universe),
    Lock(Vec<LockRow>),
    Transcript(Vec<String>),
    Text(String),
}
