//! Gate 5.1 item 6: revocation, same fact, its own check.

use super::g5_revoke;

pub(crate) fn g51_revoke() -> bool {
    g5_revoke()
}
