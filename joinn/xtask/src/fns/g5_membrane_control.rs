//! Gate 5 item 2 control: one added wire shrinks the membrane.

use super::{artifact_loads, load_calculator};
use joinn_dna::Wire;
use joinn_frame::Verdict;
use joinn_link::membrane;

pub(crate) fn g5_membrane_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok((mut body, cells)) = load_calculator() else {
        return true;
    };
    let Verdict::Ok(before) = membrane(&body, &cells) else {
        return true;
    };
    body.coding.wires.push(Wire {
        src_instance: "sum".into(),
        src_port: 2,
        dst_instance: "cli_a".into(),
        dst_port: 0,
    });
    let Verdict::Ok(after) = membrane(&body, &cells) else {
        return true;
    };
    // Control passes when the membrane does *not* shrink — the opposed failure.
    !(after.len() < before.len() && after.is_subset(&before) && after != before)
}
