//! A body's printed kind: its grove kind, or its hash's first four characters.

use joinn_frame::Hash;

use super::KINDS;

/// `calc`, `units` or `bus` for the grove's three bodies; any other body prints
/// the first four hex characters of its coding hash.
pub(crate) fn kind_name(hash: &Hash) -> String {
    let hex = hash.to_hex();
    KINDS
        .iter()
        .find(|k| k.hash_hex() == hex)
        .map_or_else(|| hash.short_hex(), |k| k.name().to_owned())
}
