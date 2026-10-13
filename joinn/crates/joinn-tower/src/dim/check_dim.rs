//! Refuse a dimension that isn't built.

use joinn_frame::Verdict;

use super::Dim;
use crate::refuse::refuse;

/// `Dim(2)` passes; any other value is refused.
pub fn check_dim(dim: Dim) -> Verdict<()> {
    if dim.0 == 2 {
        Verdict::Ok(())
    } else {
        refuse(format!(
            "tower: dimension {} is not built; acceptance is dimension 2",
            dim.0
        ))
    }
}
