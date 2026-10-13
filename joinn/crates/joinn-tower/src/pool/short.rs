//! A hash as the pool prints it.

use joinn_frame::Hash;

/// The first twelve hex digits.
pub fn short(h: &Hash) -> String {
    h.to_hex().chars().take(12).collect()
}
