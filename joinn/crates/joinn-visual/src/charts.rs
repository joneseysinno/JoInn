//! Charts: coordinate systems with a whole-unit origin in their parent, from
//! the universe chart down through galaxies and systems to bodies. A universe
//! lens is laid out on a fixed grid (plan 7.2 §2.5); the anchor is the deepest
//! chart under the viewport's centre pixel (§2.3).

mod anchor_of;
mod chart_at;
mod layout_universe;
mod rebase;

pub use layout_universe::layout_universe;

use joinn_frame::Hash;

use crate::camera::ChartId;
use crate::layout::Layout;
use crate::routes::Routes;

/// The universe chart's size.
pub const UNIVERSE_SIZE: (i64, i64) = (5504, 1600);
/// A galaxy chart's size.
pub const GALAXY_SIZE: (i64, i64) = (1296, 704);
/// A system chart's size.
pub const SYSTEM_SIZE: (i64, i64) = (304, 152);
/// The largest body surface a system slot holds.
pub const BODY_MAX: (i64, i64) = (40, 24);
/// Galaxies in a universe, four across.
pub const GALAXIES_MAX: usize = 8;
/// Systems in a galaxy, four across.
pub const SYSTEMS_MAX: usize = 16;
/// Bodies in a system, six across.
pub const BODIES_MAX: usize = 24;

/// What a chart is the chart of.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ChartKind {
    /// The root of a universe.
    Universe,
    /// A galaxy of the lens.
    Galaxy,
    /// A system of the lens.
    System,
    /// A body: its cells, ports, wires and strokes live here.
    Body,
}

/// One chart. Rectangles are half-open: `[x, x + w) × [y, y + h)`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Chart {
    /// What it is the chart of.
    pub kind: ChartKind,
    /// `universe`, a galaxy or system name, or a body alias.
    pub name: String,
    /// Its parent chart; the root is its own parent.
    pub parent: ChartId,
    /// Its origin in its parent chart, in layout units.
    pub origin: (i64, i64),
    /// Its origin in the root chart, in layout units.
    pub root_origin: (i64, i64),
    /// Width and height, in layout units.
    pub size: (i64, i64),
    /// Galaxy, system or body number in lens order, from 0.
    pub index: u32,
    /// A body's coding hash and the slot of its layout in `layouts`.
    pub body: Option<(Hash, usize)>,
}

/// A lens laid out as charts. Chart slots are depth-first in canonical order:
/// the universe, then each galaxy, its systems, and each system's bodies.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UniverseLayout {
    /// The lens laid out.
    pub lens: String,
    /// Every chart; slot 0 is the root.
    pub charts: Vec<Chart>,
    /// One layout per distinct body coding.
    pub layouts: Vec<Layout>,
    /// Every link's route in every fold state, grown with the layout.
    pub routes: Routes,
}

impl UniverseLayout {
    /// The chart in `id`'s slot.
    pub fn chart(&self, id: ChartId) -> Option<&Chart> {
        self.charts.get(id.0 as usize)
    }
}
