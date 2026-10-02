use std::fmt;

use super::Side;

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Side::Placement => "placement",
            Side::Source => "source",
            Side::Energy => "energy",
        })
    }
}
