//! Gate 3 item 5: the calculator intent set.

use super::load_calculator;
use joinn_host::{Address, intent_set};
use std::collections::BTreeSet;

pub(crate) fn g3_intent() -> bool {
    let Ok((body, cells)) = load_calculator() else {
        return false;
    };
    let want = BTreeSet::from([
        Address {
            instance: "cli_a".into(),
            port: 0,
        },
        Address {
            instance: "cli_b".into(),
            port: 0,
        },
    ]);
    intent_set(&body, &cells) == want
}
