//! Allele oracle. Panics at this boundary become refusals.

mod catching;

pub use catching::Catching;

use joinn_frame::{Value, Verdict};
use std::collections::BTreeMap;

/// A Phase 1 implementation. DNA never holds one of these.
pub trait Oracle: Send + Sync {
    /// Apply to in-port values, produce out-port values.
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>>;
}
