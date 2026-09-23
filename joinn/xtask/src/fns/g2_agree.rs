//! Distinct pointer for the phase 2 row that agrees with phase 2.1.

use super::p21_agree;

pub(crate) fn g2_agree() -> bool {
    p21_agree()
}
