use num_rational::BigRational;

use super::{Kind, Sense, Witness};
use crate::q::Q;

impl Witness {
    pub fn new(name: String, at: Q, kind: Kind, sense: Sense, value: BigRational) -> Witness {
        Witness {
            name,
            at,
            kind,
            sense,
            value,
        }
    }
}
