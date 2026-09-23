//! Coin flip.

use super::SeedRng;

impl SeedRng {
    /// Coin flip.
    pub fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }
}
