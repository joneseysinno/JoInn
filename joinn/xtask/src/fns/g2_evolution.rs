//! Phase 2 item: sum_turn is admitted against its parent.

use super::admit_sum_turn;

pub(crate) fn g2_evolution() -> bool {
    admit_sum_turn().is_ok()
}
