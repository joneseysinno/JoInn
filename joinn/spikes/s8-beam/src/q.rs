//! A tagged exact quantity, and the only operations that make one.

mod add;
mod count;
mod dot;
mod new;
mod scale;
mod total;
mod wedge;

pub use add::add;
pub use count::count;
pub use dot::dot;
pub use scale::scale;
pub use total::total;
pub use wedge::wedge;

use num_rational::BigRational;

use crate::tag::Tag;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Q {
    value: BigRational,
    tag: Tag,
}

impl Q {
    pub fn value(&self) -> &BigRational {
        &self.value
    }

    pub fn tag(&self) -> Tag {
        self.tag
    }
}

/// A plain count n/d in ℚ. A zero denominator fails to compile.
macro_rules! frac {
    ($n:expr, $d:expr) => {
        $crate::q::count(
            $n,
            const {
                match ::std::num::NonZeroU32::new($d) {
                    Some(d) => d,
                    None => panic!("a count needs a nonzero denominator"),
                }
            },
        )
    };
}

pub(crate) use frac;
