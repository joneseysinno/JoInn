//! Derived child generator.

use super::SeedRng;

impl SeedRng {
    /// Fork a child generator with a derived seed.
    pub fn fork(&mut self) -> Self {
        Self::new(self.next_u64())
    }
}
