//! Routes (plan 7.3 §2.2–2.5): links travel only on the gutters of the grid
//! layout, the empty lanes between bodies, systems and galaxies. Every
//! coordinate is an integer in layout units, in the root chart unless named.

mod grid_lines;
mod routing_graph;

pub use grid_lines::grid_lines;
pub use routing_graph::routing_graph;

/// Which charts are folded into lens nodes. Every system has one size and
/// every galaxy another, so all of a kind fold at one zoom.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Fold {
    /// Every system and galaxy open.
    Open,
    /// Every system a lens node.
    Systems,
    /// Every galaxy a lens node.
    Galaxies,
}

impl Fold {
    /// `open`, `systems folded` or `galaxies folded`.
    pub fn name(self) -> &'static str {
        match self {
            Fold::Open => "open",
            Fold::Systems => "systems folded",
            Fold::Galaxies => "galaxies folded",
        }
    }

    /// The Route table's number: 0 open, 1 systems folded, 2 galaxies folded.
    pub fn number(self) -> u32 {
        match self {
            Fold::Open => 0,
            Fold::Systems => 1,
            Fold::Galaxies => 2,
        }
    }
}

/// An axis-aligned piece of street from `a` to `b`, in root layout units.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Line {
    /// One end.
    pub a: (i64, i64),
    /// The other end.
    pub b: (i64, i64),
}

/// The routing graph: every street split at every crossing and every end.
/// Nodes are sorted by `(y, x)`, so a smaller index is a smaller `(y, x)`.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Graph {
    /// Node positions, root layout units, sorted by `(y, x)`.
    pub nodes: Vec<(i64, i64)>,
    /// Segments between neighbouring nodes, `(a, b)` with `a < b`, sorted.
    pub edges: Vec<(usize, usize)>,
    /// For each node, its neighbours and the Manhattan length to each, by
    /// neighbour index.
    pub adjacent: Vec<Vec<(usize, i64)>>,
}

impl Graph {
    /// The node at `p`, if there is one.
    pub fn node_at(&self, p: (i64, i64)) -> Option<usize> {
        self.nodes
            .binary_search_by(|n| (n.1, n.0).cmp(&(p.1, p.0)))
            .ok()
    }
}

/// A system chart's vertical gutter lines: `x = 4 + 48c`, c = 0 … 6.
pub const SYSTEM_GUTTER_X: (i64, i64, i64) = (4, 48, 7);
/// A system chart's horizontal gutter lines: `y = 12 + 32r`, r = 0 … 4.
pub const SYSTEM_GUTTER_Y: (i64, i64, i64) = (12, 32, 5);
/// A galaxy chart's vertical gutter lines: `x = 8 + 320c`, c = 0 … 4.
pub const GALAXY_GUTTER_X: (i64, i64, i64) = (8, 320, 5);
/// A galaxy chart's horizontal gutter lines: `y = 24 + 168r`, r = 0 … 4.
pub const GALAXY_GUTTER_Y: (i64, i64, i64) = (24, 168, 5);
/// The universe chart's vertical gutter lines: `x = 32 + 1360c`, c = 0 … 4.
pub const UNIVERSE_GUTTER_X: (i64, i64, i64) = (32, 1360, 5);
/// The universe chart's horizontal gutter lines: `y = 32 + 768r`, r = 0 … 2.
pub const UNIVERSE_GUTTER_Y: (i64, i64, i64) = (32, 768, 3);
/// How far a system's lines extend past its first and last lines to reach the
/// galaxy's: to `x = −8` and `x = 312`, `y = −8` and `y = 160`.
pub const SYSTEM_REACH: ((i64, i64), (i64, i64)) = ((-8, 312), (-8, 160));
/// How far a galaxy's lines extend to reach the universe's: to `x = −32` and
/// `x = 1328`, `y = −32` and `y = 736`.
pub const GALAXY_REACH: ((i64, i64), (i64, i64)) = ((-32, 1328), (-32, 736));
/// A folded system's touch points, left, right, top, bottom, in its chart:
/// each on one of its lines' extensions.
pub const SYSTEM_SIDES: [(i64, i64); 4] = [(0, 76), (304, 76), (148, 0), (148, 152)];
/// A folded galaxy's touch points, left, right, top, bottom, in its chart.
pub const GALAXY_SIDES: [(i64, i64); 4] = [(0, 360), (1296, 360), (648, 0), (648, 704)];
