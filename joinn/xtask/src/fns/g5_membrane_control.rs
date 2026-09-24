//! Gate 5 item 2 control: true when ∂ ≠ {cli_a@0, cli_b@0, sum@2}.

use super::load_calculator;
use super::subject::Subject;
use joinn_frame::Verdict;
use joinn_link::membrane;
use std::collections::BTreeSet;

pub(crate) fn g5_membrane_control(subject: &Subject) -> bool {
    let Subject::Body(body) = subject else {
        return true;
    };
    let Ok((_, cells)) = load_calculator() else {
        return true;
    };
    let expected = BTreeSet::from([
        "cli_a@0".to_string(),
        "cli_b@0".to_string(),
        "sum@2".to_string(),
    ]);
    match membrane(body, &cells) {
        Verdict::Ok(set) => {
            let got: BTreeSet<String> = set.iter().map(|a| a.address.printed()).collect();
            got != expected
        }
        Verdict::Refused(_) => true,
    }
}
