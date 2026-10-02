use std::fmt;

use super::Sense;

impl fmt::Display for Sense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Sense::Sagging => "sagging",
            Sense::Hogging => "hogging",
            Sense::Down => "down",
            Sense::Up => "up",
        })
    }
}
