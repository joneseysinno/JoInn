use super::{Axis, Side, Tag};

impl Tag {
    pub const fn new(side: Side, length: i32, axis: Axis) -> Tag {
        Tag { side, length, axis }
    }
}
