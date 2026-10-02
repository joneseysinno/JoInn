use std::fmt;

use super::Tag;

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} · length {} · {}", self.side, self.length, self.axis)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Axis, Side, Tag};

    #[test]
    fn a_tag_prints_side_length_and_axis_in_lowercase() {
        assert_eq!(Tag::MOMENT.to_string(), "source · length 1 · plane");
        assert_eq!(Tag::WORK.to_string(), "energy · length 1 · none");
        assert_eq!(Tag::DENSITY.to_string(), "source · length -1 · y");
        assert_eq!(
            Tag::new(Side::Placement, 1, Axis::X).to_string(),
            "placement · length 1 · x"
        );
    }
}
