//! A contact body's roles: derived from the surface and the forces, never
//! written. A role is a pair of two independent facts; its word is printed from
//! the pair, and there is no list of role words to add a fifth to.

mod body_roles;

pub use body_roles::body_roles;

/// Whether a cell has at least one port on the body's surface.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Facing {
    /// At least one port is on the surface.
    Out,
    /// No port is on the surface.
    In,
}

/// Whether a cell is a force's response.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Holding {
    /// A genome cell: it holds what it was given.
    Holds,
    /// A force's response: it reacts to its members.
    Reacts,
}

/// One cell's role in a contact body.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BodyRole {
    /// Out when a port is on the surface.
    pub facing: Facing,
    /// Reacts when the cell is a force's response.
    pub holding: Holding,
}

impl Facing {
    /// `faces out` or `faces in`.
    pub fn word(self) -> &'static str {
        match self {
            Facing::Out => "faces out",
            Facing::In => "faces in",
        }
    }
}

impl Holding {
    /// `holds` or `reacts`.
    pub fn word(self) -> &'static str {
        match self {
            Holding::Holds => "holds",
            Holding::Reacts => "reacts",
        }
    }
}

impl BodyRole {
    /// The word printed from the pair: protect, carry, store or respond.
    pub fn word(self) -> &'static str {
        match (self.facing, self.holding) {
            (Facing::Out, Holding::Holds) => "protect",
            (Facing::Out, Holding::Reacts) => "carry",
            (Facing::In, Holding::Holds) => "store",
            (Facing::In, Holding::Reacts) => "respond",
        }
    }
}
