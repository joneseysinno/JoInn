use std::fmt;

use super::Axis;

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Axis::None => "none",
            Axis::X => "x",
            Axis::Y => "y",
            Axis::Plane => "plane",
        })
    }
}
