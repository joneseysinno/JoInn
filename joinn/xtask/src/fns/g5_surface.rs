//! Gate 5 item 2: ∂(calculator) is the three surface ports, by value.

use super::load_calculator;
use joinn_frame::Verdict;
use joinn_link::surface;
use std::collections::BTreeSet;

pub(crate) fn g5_surface() -> bool {
    let Ok((body, cells)) = load_calculator() else {
        return false;
    };
    let Verdict::Ok(set) = surface(&body, &cells) else {
        return false;
    };
    let got: BTreeSet<String> = set.iter().map(|a| a.address.printed()).collect();
    got == BTreeSet::from([
        "cli_a@0".to_string(),
        "cli_b@0".to_string(),
        "sum@2".to_string(),
    ])
}
