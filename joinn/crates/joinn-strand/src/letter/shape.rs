//! A letter's shape: the low bit.

use super::{Letter, Shape};

impl Letter {
    /// Big (A, G) or small (T, C).
    pub fn shape(self) -> Shape {
        if self.bits() & 0b01 == 0 {
            Shape::Big
        } else {
            Shape::Small
        }
    }
}
