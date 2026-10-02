//! The witnesses: AISC Manual Table 3-23 and Roark Table 8.1, each formula
//! stated once as an exact expression of an example's inputs. A witness is
//! compared with a derived value; it is never used to derive one.

mod compare;
mod new;
mod sense_word;
mod stated;

pub use compare::compare;
pub use stated::stated;

use num_rational::BigRational;

use crate::q::Q;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// M, compared in kip·in and printed in kip·ft.
    Moment,
    /// v, in inches.
    Deflection,
}

/// The case's sense, which a witness states beside its magnitude.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sense {
    Sagging,
    Hogging,
    Down,
    Up,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Witness {
    name: String,
    at: Q,
    kind: Kind,
    sense: Sense,
    /// The magnitude, in kip·in (moment) or in (deflection).
    value: BigRational,
}

impl Witness {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn at(&self) -> &Q {
        &self.at
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn value(&self) -> &BigRational {
        &self.value
    }
}
