//! Phase 2 item: the calculator bodies share one canonical coding region.

use super::verify_phase2_bodies;

pub(crate) fn g2_bodies() -> bool {
    verify_phase2_bodies().is_ok()
}
