//! Gate 5.1 item 3: a declared hash that is not the supplied body is refused.

use super::{load_phase5_bodies, workspace_root};
use joinn_dna::hash;
use joinn_frame::Verdict;
use joinn_link::{bind_bodies, parse_universe};
use std::fs;

pub(crate) fn g51_hash() -> bool {
    let Ok(root) = workspace_root() else {
        return false;
    };
    let Ok(src) = fs::read_to_string(
        root.join("corpus")
            .join("phase5")
            .join("controls")
            .join("wrong_hash.universe"),
    ) else {
        return false;
    };
    let Verdict::Ok(universe) = parse_universe(&src) else {
        return false;
    };
    let Ok(mut supplied) = load_phase5_bodies() else {
        return false;
    };
    let Some(declared) = universe.coding.bodies.first().map(|b| b.hash) else {
        return false;
    };
    let Some((calc, cells)) = supplied
        .values()
        .find(|(body, _)| hash(&body.coding) != declared)
    else {
        return false;
    };
    let calc = calc.clone();
    let cells = cells.clone();
    let got = hash(&calc.coding);
    supplied.insert(declared, (calc, cells));
    match bind_bodies(&universe, &supplied) {
        Verdict::Refused(r) => {
            r.reason.contains("calc")
                && r.reason.contains(&declared.short_hex())
                && r.reason.contains(&got.short_hex())
        }
        Verdict::Ok(_) => false,
    }
}
