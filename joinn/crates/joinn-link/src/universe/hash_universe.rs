//! Domain-tagged hash of a universe's canonical coding text.

use joinn_frame::{Hash, TAG_UNIVERSE, keyed_hash};

use super::UniverseCoding;
use super::print_universe::print_universe;

/// Domain-tagged hash of a universe's canonical coding text.
pub fn hash_universe(coding: &UniverseCoding) -> Hash {
    keyed_hash(TAG_UNIVERSE, print_universe(coding).as_bytes())
}
