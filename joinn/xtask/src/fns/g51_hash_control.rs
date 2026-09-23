//! Gate 5.1 item 3 control: the artifact names calc with the other body's hash.

use super::artifact_loads;
use joinn_frame::Verdict;
use joinn_link::parse_universe;

pub(crate) fn g51_hash_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok(text) = std::str::from_utf8(art.bytes) else {
        return true;
    };
    let Verdict::Ok(universe) = parse_universe(text) else {
        return true;
    };
    let Some(binding) = universe.coding.bodies.iter().find(|b| b.alias == "calc") else {
        return true;
    };
    let declared = binding.hash.to_hex();
    declared.starts_with("b55f")
}
