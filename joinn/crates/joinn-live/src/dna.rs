//! Runs a DNA body as a cell's allele.

mod fire;

use joinn_gate::NativeRegistry;

/// Runs a DNA body as a cell's allele. The live engine's `DnaFire`.
pub struct LiveDna {
    natives: NativeRegistry,
    last_steps: std::cell::Cell<u64>,
}

impl LiveDna {
    /// Construct from the native register the engine will use.
    pub fn new(natives: NativeRegistry) -> Self {
        Self {
            natives,
            last_steps: std::cell::Cell::new(0),
        }
    }
}
