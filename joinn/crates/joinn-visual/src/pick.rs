//! Exact CPU pick, the reference allele of drawing. Integer arithmetic only.
//!
//! A pixel `(x, y)` is judged at its centre. Every coordinate is taken in
//! sixteenths of a pixel, so the centre is `(16x + 8, 16y + 8)`. The edge band is
//! one sixteenth: a shape with signed distance `d` holds a pixel inside when
//! `d ≤ −1`, outside when `d ≥ 1`, and at its edge otherwise.

mod bounds16;
mod capsule_class;
mod circle_class;
mod class_at;
mod cpu_pick;
mod cpu_pick_reference;
mod geom16;
mod owner_of_layout;
mod print_owner;
mod round_rect_class;
mod shapes_of_layout;
mod squared_class;

pub use cpu_pick::cpu_pick;
pub use cpu_pick_reference::cpu_pick_reference;
pub use owner_of_layout::owner_of_layout;
pub use print_owner::print_owner;
pub use shapes_of_layout::shapes_of_layout;

use joinn_link::Address;

/// Blue channel of a port's ID: `PORT_TAG | position`.
pub const PORT_TAG: u32 = 0x1000_0000;
/// Blue channel of a wire's ID: `WIRE_TAG | link slot`.
pub const WIRE_TAG: u32 = 0x2000_0000;
/// The tag bits of the blue channel.
pub const TAG_MASK: u32 = 0xF000_0000;

/// A shape's geometry in layout units.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Geom {
    /// A rounded rectangle `x y w h` with corner radius `r` (surfaces, cells).
    RoundRect {
        /// Left.
        x: i64,
        /// Top.
        y: i64,
        /// Width.
        w: i64,
        /// Height.
        h: i64,
        /// Corner radius.
        r: i64,
    },
    /// A circle (ports).
    Circle {
        /// Centre x.
        x: i64,
        /// Centre y.
        y: i64,
        /// Radius.
        r: i64,
    },
    /// A capsule from `a` to `b` (wires). Half-width in quarter units.
    Capsule {
        /// Start.
        a: (i64, i64),
        /// End.
        b: (i64, i64),
        /// Half-width in quarters of a layout unit.
        half_quarters: i64,
    },
}

/// One drawn shape and the ID it writes to the ID target (R G B A, §2.8).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Shape {
    /// Geometry in layout units.
    pub geom: Geom,
    /// The ID texel this shape owns.
    pub id: [u32; 4],
}

/// Geometry in sixteenths of a pixel, after the camera.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Geom16 {
    RoundRect {
        c: (i128, i128),
        h: (i128, i128),
        r: i128,
    },
    Circle {
        c: (i128, i128),
        r: i128,
    },
    Capsule {
        a: (i128, i128),
        b: (i128, i128),
        w: i128,
    },
}

/// Where a pixel centre lies relative to one shape.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Class {
    /// `d ≤ −1`.
    Inside,
    /// `−1 < d < 1`: counted, never judged.
    Edge,
    /// `d ≥ 1`.
    Outside,
}

/// What a pixel shows.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pick {
    /// No shape holds it.
    Background,
    /// The topmost shape that isn't outside has it on its edge.
    Edge,
    /// The topmost shape that isn't outside holds it inside.
    Owned([u32; 4]),
}

/// Every pixel of a viewport, row by row.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PickImage {
    /// Pixels per row.
    pub width: u32,
    /// Rows.
    pub height: u32,
    /// `width × height` picks, row-major.
    pub pixels: Vec<Pick>,
}

/// Who owns a pixel, by name.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Owner {
    /// The clear colour.
    Background,
    /// The body's surface.
    Surface,
    /// An instance's cell face.
    Cell(String),
    /// A port.
    Port(Address),
    /// A wire.
    Wire {
        /// Source out-port.
        src: Address,
        /// Destination in-port.
        dst: Address,
    },
}
