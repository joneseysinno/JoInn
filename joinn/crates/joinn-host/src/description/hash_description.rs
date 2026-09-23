//! Domain-tagged hash of a description's canonical form.

use joinn_frame::{Hash, TAG_DESCRIPTION, keyed_hash};

use super::Description;
use super::print_description::print_description;

/// Domain-tagged hash of a description's canonical form.
pub fn hash_description(d: &Description) -> Hash {
    keyed_hash(TAG_DESCRIPTION, print_description(d).as_bytes())
}
