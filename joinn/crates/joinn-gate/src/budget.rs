//! Sampling budget. Every refusal reports the seed.

use joinn_frame::DEFAULT_SIZE;

/// How many samples, from which seed, at what size.
#[derive(Clone, Copy, Debug)]
pub struct Budget {
    /// Explicit seed. Replays exactly.
    pub seed: u64,
    /// `ForAll` draws this many tuples.
    pub law_samples: u32,
    /// Testimony replay sample count.
    pub testimony_samples: u32,
    /// Generator size.
    pub size: u8,
}

impl Default for Budget {
    fn default() -> Self {
        Self {
            seed: 1,
            law_samples: 64,
            testimony_samples: 32,
            size: DEFAULT_SIZE,
        }
    }
}
