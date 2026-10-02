//! Where a quantity lives (Part VI §8, in miniature): its side, its length
//! (counting), and its axis in the elevation plane, x along the span and y up.

mod axis_word;
mod display;
mod new;
mod side_word;
mod unit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Placement,
    Source,
    /// A source · placement product, the shared currency.
    Energy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    None,
    X,
    Y,
    /// x ∧ y.
    Plane,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tag {
    side: Side,
    length: i32,
    axis: Axis,
}

impl Tag {
    /// x, L, a lever arm, a segment length ℓ.
    pub const PLACE_X: Tag = Tag::new(Side::Placement, 1, Axis::X);
    /// P, R, V.
    pub const FORCE: Tag = Tag::new(Side::Source, 0, Axis::Y);
    /// The oriented load q = −w.
    pub const DENSITY: Tag = Tag::new(Side::Source, -1, Axis::Y);
    /// M.
    pub const MOMENT: Tag = Tag::new(Side::Source, 1, Axis::Plane);
    /// Force · displacement along y.
    pub const WORK: Tag = Tag::new(Side::Energy, 1, Axis::None);
    /// θ.
    pub const ROTATION: Tag = Tag::new(Side::Placement, 0, Axis::Plane);
    /// v.
    pub const DEFLECTION: Tag = Tag::new(Side::Placement, 1, Axis::Y);
    /// κ = M / EI.
    pub const CURVATURE: Tag = Tag::new(Side::Placement, -1, Axis::Plane);

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn length(&self) -> i32 {
        self.length
    }

    pub fn axis(&self) -> Axis {
        self.axis
    }
}
