//! Gate 5.1 item 1: the same crossing, its own check.

use super::g5_linked;

pub(crate) fn g51_cross() -> bool {
    g5_linked()
}
