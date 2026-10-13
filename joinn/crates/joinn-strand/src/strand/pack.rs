//! Pack a strand four letters to a byte.

use super::{Packed, Strand};

impl Strand {
    /// Letter `i` sits in byte `i / 4` at bits `6 − 2·(i mod 4)`; the unused
    /// low bits of a last partial byte are zero.
    pub fn pack(&self) -> Packed {
        let mut bytes = vec![0u8; self.letters.len().div_ceil(4)];
        for (i, l) in self.letters.iter().enumerate() {
            if let Some(b) = bytes.get_mut(i / 4) {
                *b |= l.bits() << (6 - 2 * (i % 4));
            }
        }
        Packed {
            bytes,
            len: self.letters.len() as u64,
        }
    }
}
