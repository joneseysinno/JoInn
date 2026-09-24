//! Phase 2 turn control: a result of 5 would mean the turn was not applied.

use super::{int_val};
use joinn_frame::Verdict;

pub(crate) fn g2_turn_control(_art: &()) -> bool {
    let Ok(five) = int_val(5) else {
        return true;
    };
    matches!(joinn_prim::five_and_three_at_turn(), Verdict::Ok(v) if v == five)
}
