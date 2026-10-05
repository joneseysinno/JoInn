//! `cargo xtask pick`: the GPU ID target against the exact CPU pick, on every
//! pixel, on every adapter.

mod compare;
mod contact_lines;
mod contact_subjects;
mod frame_median;
mod grove_lines;
mod grove_subjects;
mod measured;
mod owner_name;
mod plant;
mod port_slot;
mod run;

pub(crate) use contact_lines::contact_lines;
pub(crate) use contact_subjects::contact_subjects;
pub(crate) use grove_lines::grove_lines;
pub(crate) use grove_subjects::grove_subjects;
pub(crate) use owner_name::owner_name;
pub(crate) use port_slot::port_slot;
pub(crate) use run::pick;

use std::collections::BTreeSet;

use joinn_visual::{Camera, FitCamera, PickImage, Scene};

/// Seeded pixels per grove view (plan 7.2 2.10).
pub(crate) const GROVE_SAMPLE: usize = 4096;

/// One plan 7.2 2.12 view of the grove: its camera, the scene grown at its
/// anchor, the grid pick of the whole image, the sampled pixel indices, how
/// many sampled pixels the brute-force walk names differently, and every owner
/// of a pixel in the grid pick.
pub(crate) struct GroveView {
    pub(crate) label: String,
    pub(crate) camera: Camera,
    pub(crate) scene: usize,
    pub(crate) cpu: PickImage,
    pub(crate) sample: Vec<usize>,
    pub(crate) differ: usize,
    pub(crate) allows: BTreeSet<[u32; 4]>,
}

/// A measured corpus body at 1280×720 with its CPU pick, or why it isn't measured.
pub(crate) type Subject = (String, Result<(Scene, FitCamera, PickImage), String>);

/// A corpus contact's file name, its grown scene, and its CPU pick at each
/// standard viewport.
pub(crate) type ContactSubject = (String, Scene, Vec<(FitCamera, PickImage)>);

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
