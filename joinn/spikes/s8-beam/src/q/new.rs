use num_rational::BigRational;

use super::Q;
use crate::tag::Tag;

impl Q {
    pub fn new(value: BigRational, tag: Tag) -> Q {
        Q { value, tag }
    }
}
