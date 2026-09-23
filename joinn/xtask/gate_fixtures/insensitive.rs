//! A control that ignores the bytes it is given.
//! The gate run must refuse this item: damaging the file does not change the answer.

use joinn_gate::Artifact;

pub fn ignores_its_bytes(_art: &Artifact) -> bool {
    false
}
