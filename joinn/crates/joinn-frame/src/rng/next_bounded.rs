//! Bounded sample.

use super::SeedRng;

impl SeedRng {
    /// Uniform in `0..max`. Returns 0 when `max` is 0.
    pub fn next_bounded(&mut self, max: u64) -> u64 {
        if max == 0 { 0 } else { self.next_u64() % max }
    }
}
