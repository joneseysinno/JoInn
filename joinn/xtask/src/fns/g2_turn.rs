//! Phase 2 item: 5 and 3 at Sum produce 2.

use super::int_val;
use joinn_frame::Verdict;

pub(crate) fn g2_turn() -> bool {
    match joinn_prim::five_and_three_at_turn() {
        Verdict::Ok(v) => match int_val(2) {
            Ok(two) if v == two => {
                println!("turn: 5 and 3 at Sum produced 2");
                true
            }
            _ => false,
        },
        Verdict::Refused(_) => false,
    }
}
