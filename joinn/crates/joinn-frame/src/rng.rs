//! Seeded SplitMix64. No ambient entropy.

mod fork;
mod next_bool;
mod next_bounded;
mod next_u64;

/// Deterministic generator. A refusal must report the seed that produced it.
#[derive(Clone, Debug)]
pub struct SeedRng {
    pub(in crate::rng) state: u64,
}

impl SeedRng {
    /// Create from an explicit seed.
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}
