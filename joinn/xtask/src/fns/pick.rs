//! `cargo xtask pick`: the GPU ID target against the exact CPU pick, on every
//! pixel, on every adapter.

mod compare;
mod frame_median;
mod measured;
mod owner_name;
mod plant;
mod port_slot;
mod run;

pub(crate) use run::pick;

use std::collections::BTreeSet;

use joinn_visual::{Camera, PickImage, Scene};

/// A measured corpus body at 1280×720 with its CPU pick, or why it isn't measured.
pub(crate) type Subject = (String, Result<(Scene, Camera, PickImage), String>);

/// Disagreements kept for printing, per comparison.
pub(crate) const KEPT: usize = 5;

/// One GPU ID image against one CPU pick image. `agree` counts non-edge pixels
/// where both name the same owner; `agree + edge + disagree` is every pixel.
#[derive(Clone, Debug, Default)]
pub(crate) struct Tally {
    pub(crate) agree: usize,
    pub(crate) edge: usize,
    pub(crate) disagree: usize,
    /// Every owner of an agreeing non-background pixel.
    pub(crate) owners: BTreeSet<[u32; 4]>,
    /// The first disagreements: pixel, CPU ID (zeros for background), GPU ID.
    pub(crate) first: Vec<(usize, [u32; 4], [u32; 4])>,
}
