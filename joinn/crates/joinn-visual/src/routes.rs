//! Routes (plan 7.3 §2.2–2.5): links travel only on the gutters of the grid
//! layout, the empty lanes between bodies, systems and galaxies. Every
//! coordinate is an integer in layout units, in the root chart unless named.

mod crossings;
mod distances;
mod fold_at;
mod form_of;
mod grid_lines;
mod half_width;
mod knot;
mod route_link;
mod routes_of;
mod routing_graph;
mod side;
mod stub;
mod walk;

pub use crossings::crossings;
pub use fold_at::fold_at;
pub use form_of::form_of;
pub use grid_lines::grid_lines;
pub use half_width::half_width;
pub use routes_of::routes;
pub use routing_graph::routing_graph;
pub use side::side;

use crate::camera::ChartId;

/// Sixteenths per layout unit: route pieces are placed in sixteenths, so a
/// path's midpoint and every half-width are whole numbers.
pub const SIXTEENTHS: i64 = 16;

/// What a route piece is.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PieceKind {
    /// Street, from the knot toward a touch point, or a spine's path.
    Leg,
    /// From a port's centre out to its gutter.
    Stub,
    /// The knot: `a == b`.
    Knot,
    /// An arrowhead from its base centre `a` to its tip `b`.
    Arrow,
}

impl PieceKind {
    /// The Segment table's number: 0 leg, 1 stub, 2 knot, 3 arrow.
    pub fn number(self) -> u32 {
        match self {
            PieceKind::Leg => 0,
            PieceKind::Stub => 1,
            PieceKind::Knot => 2,
            PieceKind::Arrow => 3,
        }
    }
}

/// One drawn piece of a route, from `a` to `b` in root sixteenths.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Piece {
    /// One end (an arrow's base centre).
    pub a: (i64, i64),
    /// The other end (an arrow's tip).
    pub b: (i64, i64),
    /// What it is.
    pub kind: PieceKind,
    /// The member it serves, index + 1, or 0 for trunk, knot and spine.
    pub member: u32,
    /// How many legs share it (1 for stubs, arrows and spines, 0 for a knot).
    pub legs: u32,
}

/// Where a link touches one node of a fold state.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Touch {
    /// The body (open) or the folded system or galaxy.
    pub chart: ChartId,
    /// Its touch point, root layout units: a port's centre, or the side of a
    /// folded node that faces the knot.
    pub point: (i64, i64),
    /// The link's members it stands for, by index in the link.
    pub members: Vec<usize>,
    /// The graph node a leg ends at: the stub's end on its gutter, or the
    /// touch point itself.
    pub node: (i64, i64),
}

/// One link's route in one fold state.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Route {
    /// The link's index in the universe's links.
    pub link: usize,
    /// The fold state.
    pub fold: Fold,
    /// Declared order: drawn as a spine.
    pub ordered: bool,
    /// One per node touched, in member order (V151).
    pub touches: Vec<Touch>,
    /// The knot of an unordered link with two touch points or more.
    pub knot: Option<(i64, i64)>,
    /// Legs drawn: one per touch when there is a knot, else none.
    pub legs: usize,
    /// Stubs drawn: one per member whose body is drawn.
    pub stubs: usize,
    /// `max(w, h)` of the box of its touch points and knot, layout units.
    pub size: i64,
    /// Every piece, legs and spine first, then stubs, arrows and the knot.
    pub pieces: Vec<Piece>,
}

/// Every link's route in every fold state the zoom range reaches, and each
/// fold state's routing graph (stub ends included).
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Routes {
    /// Fold states, open first.
    pub folds: Vec<Fold>,
    /// Each fold state's graph, in `folds` order.
    pub graphs: Vec<Graph>,
    /// Routes by fold state, then link.
    pub routes: Vec<Route>,
}

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

/// How a link is drawn at a view (§2.5): by the owner rule over its projected
/// size, or a spine when its order is declared.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Form {
    /// A filled region, half-width 3, below 264 px.
    Region,
    /// Thin legs and a knot disc, from 264 px.
    Hub,
    /// Legs widened by how many share a segment, from 2112 px.
    Bundle,
    /// Declared order: one path through the members, with arrowheads.
    Spine,
}

impl Form {
    /// `region`, `hub`, `bundle` or `spine`.
    pub fn name(self) -> &'static str {
        match self {
            Form::Region => "region",
            Form::Hub => "hub",
            Form::Bundle => "bundle",
            Form::Spine => "spine",
        }
    }

    /// The number in a link pixel's ID: 0 region, 1 hub, 2 bundle, 3 spine.
    pub fn number(self) -> u32 {
        match self {
            Form::Region => 0,
            Form::Hub => 1,
            Form::Bundle => 2,
            Form::Spine => 3,
        }
    }
}

/// Form thresholds in pixels: region to hub at 240, hub to bundle at 1920.
pub const FORM_THRESHOLDS: [i64; 2] = [240, 1920];

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
