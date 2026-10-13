//! The owner of a strand's or a fold's pixel.

use crate::pick::{CODON_TAG, TAG_MASK};

/// `[1, part, CODON_TAG | index]`, or `None` when `index` reaches the tag bits.
pub(crate) fn owner(part: u32, index: u64) -> Option<[u32; 3]> {
    let i = u32::try_from(index).ok().filter(|i| i & TAG_MASK == 0)?;
    Some([1, part, CODON_TAG | i])
}
