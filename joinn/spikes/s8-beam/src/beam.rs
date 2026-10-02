//! A beam in its elevation plane: x along the span, y up. Up is positive and
//! sagging is positive; "down", "sagging" and "hogging" are printed words.

mod balance;
mod closure;
mod complex;
mod deflect;
mod deflected_at;
mod derive;
mod examples;
mod refinement;
mod refinement_of_deflection;
mod station_at;
mod walk;

pub use balance::balance;
pub use closure::closure;
pub use complex::complex;
pub use deflect::deflect;
pub use deflected_at::deflected_at;
pub use derive::derive;
pub use examples::examples;
pub use refinement::refinement;
pub use refinement_of_deflection::refinement_of_deflection;
pub use station_at::station_at;
pub use walk::walk;

use crate::q::Q;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Support {
    /// Pin at x = 0, roller at x = L.
    Simple,
    /// Fixed at x = 0, free at x = L.
    Cantilever,
}

/// A force along y (source · length 0 · y) acting at x.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PointLoad {
    at: Q,
    force: Q,
}

/// One worked example: its supports, span, loads, and the sections it prints.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Example {
    id: &'static str,
    title: &'static str,
    support: Support,
    span: Q,
    /// The oriented load q = −w over the whole span.
    uniform: Option<Q>,
    point_loads: Vec<PointLoad>,
    moment_sections: Vec<Q>,
    deflection_sections: Vec<Q>,
}

/// A point of the complex and the point loads acting at it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Point {
    x: Q,
    loads: Vec<Q>,
}

/// A line between two neighboring points, with the oriented load on it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    length: Q,
    q: Q,
}

/// The elevation complex: points sorted by x, one line between neighbors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Complex {
    points: Vec<Point>,
    lines: Vec<Line>,
}

/// Balance's result: the reactions, and ΣF and ΣM with them in place.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Balance {
    r_a: Q,
    r_b: Option<Q>,
    /// The wall's couple C_A (cantilever only).
    couple: Option<Q>,
    sum_f: Q,
    sum_m: Q,
}

/// How the walk takes each load's moment. Only `Faithful` is the derivation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Order {
    Faithful,
    /// Plant: the load's moment as F ∧ r while the reactions stay r ∧ F.
    MixedOrder,
    /// Plant: the load's moment term omitted, a rectangle rule.
    Rectangle,
}

/// The walk at one point: M there, and V just right of it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Station {
    x: Q,
    m: Q,
    v: Q,
}

/// Rotation θ and deflection v at one point of the complex.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Deflected {
    x: Q,
    theta: Q,
    v: Q,
}

impl Deflected {
    pub fn x(&self) -> &Q {
        &self.x
    }

    pub fn theta(&self) -> &Q {
        &self.theta
    }

    pub fn v(&self) -> &Q {
        &self.v
    }
}

/// One example derived: its complex, balance, walk, the closure's M at the
/// far end, and the bridge reads those steps made.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Derived {
    complex: Complex,
    balance: Balance,
    stations: Vec<Station>,
    end: Q,
    reads_before_bridge: u32,
}

impl Derived {
    pub fn complex(&self) -> &Complex {
        &self.complex
    }

    pub fn balance(&self) -> &Balance {
        &self.balance
    }

    pub fn stations(&self) -> &[Station] {
        &self.stations
    }

    pub fn end(&self) -> &Q {
        &self.end
    }

    pub fn reads_before_bridge(&self) -> u32 {
        self.reads_before_bridge
    }
}

impl PointLoad {
    pub fn at(&self) -> &Q {
        &self.at
    }

    pub fn force(&self) -> &Q {
        &self.force
    }
}

impl Example {
    pub fn id(&self) -> &'static str {
        self.id
    }

    pub fn title(&self) -> &'static str {
        self.title
    }

    pub fn support(&self) -> Support {
        self.support
    }

    pub fn span(&self) -> &Q {
        &self.span
    }

    pub fn uniform(&self) -> Option<&Q> {
        self.uniform.as_ref()
    }

    pub fn point_loads(&self) -> &[PointLoad] {
        &self.point_loads
    }

    pub fn moment_sections(&self) -> &[Q] {
        &self.moment_sections
    }

    pub fn deflection_sections(&self) -> &[Q] {
        &self.deflection_sections
    }
}

impl Point {
    pub fn x(&self) -> &Q {
        &self.x
    }
}

impl Line {
    pub fn length(&self) -> &Q {
        &self.length
    }

    pub fn q(&self) -> &Q {
        &self.q
    }
}

impl Complex {
    pub fn points(&self) -> &[Point] {
        &self.points
    }

    pub fn lines(&self) -> &[Line] {
        &self.lines
    }
}

impl Balance {
    pub fn r_a(&self) -> &Q {
        &self.r_a
    }

    pub fn r_b(&self) -> Option<&Q> {
        self.r_b.as_ref()
    }

    pub fn couple(&self) -> Option<&Q> {
        self.couple.as_ref()
    }

    pub fn sum_f(&self) -> &Q {
        &self.sum_f
    }

    pub fn sum_m(&self) -> &Q {
        &self.sum_m
    }
}

impl Station {
    pub fn x(&self) -> &Q {
        &self.x
    }

    pub fn m(&self) -> &Q {
        &self.m
    }

    pub fn v(&self) -> &Q {
        &self.v
    }
}
