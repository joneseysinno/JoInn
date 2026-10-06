//! The fold state a zoom shows.

use super::Fold;
use crate::bands::{Band, owner_band};
use crate::camera::Zoom;
use crate::charts::{GALAXY_SIZE, SYSTEM_SIZE};

/// Galaxies folded when a galaxy's owner band is below Summary, else systems
/// folded when a system's is, else open: the cut's rule for a lens node.
pub fn fold_at(zoom: Zoom) -> Fold {
    let below = |size: (i64, i64)| owner_band(zoom, size.0.max(size.1)) < Band::Summary;
    if below(GALAXY_SIZE) {
        Fold::Galaxies
    } else if below(SYSTEM_SIZE) {
        Fold::Systems
    } else {
        Fold::Open
    }
}

#[cfg(test)]
mod tests {
    use super::fold_at;
    use crate::camera::Zoom;
    use crate::routes::Fold;

    #[test]
    fn systems_fold_at_the_lowest_level_and_open_one_level_up() {
        let z = |level| Zoom { level, step: 0 };
        assert_eq!(fold_at(z(-4)), Fold::Systems);
        assert_eq!(fold_at(z(-3)), Fold::Open);
        assert_eq!(fold_at(z(9)), Fold::Open);
    }
}
