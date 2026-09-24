//! Gate 5 item 5 control: true when fewer than two lenses place units.

use super::subject::Subject;

pub(crate) fn g5_lenses_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return true;
    };
    let placing = u.coding.lenses.iter().filter(|lens| {
        lens.galaxies.iter().any(|galaxy| {
            galaxy
                .systems
                .iter()
                .any(|system| system.bodies.iter().any(|a| a == "units"))
        })
    }).count();
    placing < 2
}
