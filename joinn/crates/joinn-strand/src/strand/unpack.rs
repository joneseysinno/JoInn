//! Unpack a packed strand.

use super::{Packed, Strand};
use crate::letter::Letter;

impl Packed {
    /// Exactly `len` letters; bits past them are not read.
    pub fn unpack(&self) -> Strand {
        let letters = (0..self.len)
            .map_while(|i| {
                let i = usize::try_from(i).ok()?;
                let b = self.bytes.get(i / 4)?;
                Some(Letter::from_bits(b >> (6 - 2 * (i % 4))))
            })
            .collect();
        Strand::new(letters)
    }
}
