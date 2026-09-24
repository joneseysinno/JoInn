//! Gate 5 item 2 control: one added wire shrinks the membrane.

use super::load_calculator;
use super::subject::Subject;
use joinn_dna::Wire;
use joinn_frame::Verdict;
use joinn_link::membrane;

pub(crate) fn g5_membrane_control(subject: &Subject) -> bool {
    let Subject::Body(body) = subject else {
        return true;
    };
    let Ok((_, cells)) = load_calculator() else {
        return true;
    };
    let Verdict::Ok(before) = membrane(body, &cells) else {
        return true;
    };
    let mut mutant = body.clone();
    mutant.coding.wires.push(Wire {
        src_instance: "sum".into(),
        src_port: 2,
        dst_instance: "cli_a".into(),
        dst_port: 0,
    });
    let Verdict::Ok(after) = membrane(&mutant, &cells) else {
        return true;
    };
    // Control passes when the membrane does *not* shrink - the opposed failure.
    !(after.len() < before.len() && after.is_subset(&before) && after != before)
}
