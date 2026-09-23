//! Domain-tagged BLAKE3.

use super::Hash;

/// BLAKE3-256 over a length-prefixed domain tag and payload.
pub fn keyed_hash(tag: &[u8], payload: &[u8]) -> Hash {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&(tag.len() as u64).to_le_bytes());
    hasher.update(tag);
    hasher.update(&(payload.len() as u64).to_le_bytes());
    hasher.update(payload);
    Hash(*hasher.finalize().as_bytes())
}
