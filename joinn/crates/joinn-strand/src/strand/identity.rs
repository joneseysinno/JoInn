//! A strand's identity.

use joinn_frame::{Hash, keyed_hash};

use super::{Strand, TAG_DNA};

impl Strand {
    /// JoInn's hash under `joinn.dna.v1` over the length in letters (eight
    /// bytes, little-endian) and the packed bytes.
    pub fn identity(&self) -> Hash {
        let packed = self.pack();
        let mut payload = packed.len().to_le_bytes().to_vec();
        payload.extend_from_slice(packed.bytes());
        keyed_hash(TAG_DNA, &payload)
    }
}
