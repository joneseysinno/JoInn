//! Convenience: the ℚ add allele on `Sum`.

use joinn_dna::{Allele, AlleleBody, NativeId};
use joinn_frame::FrameRef;

/// Convenience: the ℚ add allele on `Sum`.
pub fn add_rat_allele() -> Allele {
    Allele {
        frame: FrameRef::rat(),
        body: AlleleBody::Native(NativeId("add@ℚ".into())),
        witnesses: Vec::new(),
    }
}
