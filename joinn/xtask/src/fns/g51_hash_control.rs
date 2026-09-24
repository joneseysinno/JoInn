//! Gate 5.1 item 3 control: the artifact names calc with the other body's hash.

use super::subject::Subject;

pub(crate) fn g51_hash_control(subject: &Subject) -> bool {
    let Subject::Universe(universe) = subject else {
        return true;
    };
    let Some(binding) = universe.coding.bodies.iter().find(|b| b.alias == "calc") else {
        return true;
    };
    let declared = binding.hash.to_hex();
    declared.starts_with("b55f")
}
