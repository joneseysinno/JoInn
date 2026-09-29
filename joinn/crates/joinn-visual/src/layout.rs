//! A body's layout: a pure function of its coding region and its cells' contracts.
//! Every number is an integer in layout units.

mod columns;
mod layout_body;
mod print_layout;

pub use layout_body::layout;
pub use print_layout::print_layout;

use joinn_dna::Direction;
use joinn_link::Address;

/// Surface to cells.
pub const MARGIN: i64 = 4;
/// Width of every cell.
pub const CELL_W: i64 = 12;
/// Between columns.
pub const GAP_X: i64 = 8;
/// Between cells in a column.
pub const GAP_Y: i64 = 4;
/// Between ports on one side of a cell.
pub const PITCH: i64 = 4;
/// From a cell's top to its first port centre, and from its last port centre to its bottom.
pub const PORT_INSET: i64 = 3;
/// Port radius.
pub const PORT_RADIUS: i64 = 1;
/// Cell corner radius.
pub const CELL_RADIUS: i64 = 2;
/// Surface corner radius.
pub const SURFACE_RADIUS: i64 = 3;
/// Wire half-width, in quarters of a layout unit.
pub const WIRE_HALF_WIDTH_QUARTERS: i64 = 1;

/// A rectangle: `x y width height`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rect {
    /// Left edge.
    pub x: i64,
    /// Top edge.
    pub y: i64,
    /// Width.
    pub w: i64,
    /// Height.
    pub h: i64,
}

/// One instance's cell.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CellBox {
    /// Instance name.
    pub instance: String,
    /// Its rectangle.
    pub rect: Rect,
}

/// One port's centre. In-ports sit on a cell's left edge, out-ports on its right.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PortDot {
    /// Instance and position.
    pub address: Address,
    /// In or out.
    pub direction: Direction,
    /// Centre x.
    pub x: i64,
    /// Centre y.
    pub y: i64,
}

/// One wire, from a source out-port's centre to a destination in-port's centre.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WireSeg {
    /// Source out-port.
    pub src: Address,
    /// Destination in-port.
    pub dst: Address,
    /// Source centre.
    pub from: (i64, i64),
    /// Destination centre.
    pub to: (i64, i64),
}

/// A body's layout. Cells in name order, ports by instance then position, wires
/// in the order `print_body` prints them.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Layout {
    /// The surface, from `(0, 0)`.
    pub surface: Rect,
    /// Cells.
    pub cells: Vec<CellBox>,
    /// Ports.
    pub ports: Vec<PortDot>,
    /// Wires.
    pub wires: Vec<WireSeg>,
}
