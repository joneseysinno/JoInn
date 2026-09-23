//! Gate 5 item 5: units sits in two systems across two lenses.

use super::load_universe_file;
use joinn_frame::Verdict;
use joinn_link::check_lenses;
use std::collections::BTreeSet;

pub(crate) fn g5_lenses() -> bool {
    let Ok(u) = load_universe_file("phase5/universe.universe") else {
        return false;
    };
    match check_lenses(&u) {
        Verdict::Ok(()) => {}
        Verdict::Refused(_) => return false,
    }
    let mut systems = BTreeSet::new();
    for lens in &u.coding.lenses {
        let mut found = false;
        for galaxy in &lens.galaxies {
            for system in &galaxy.systems {
                if system.bodies.iter().any(|a| a == "units") {
                    if found {
                        return false;
                    }
                    found = true;
                    systems.insert(system.name.clone());
                }
            }
        }
        if !found {
            return false;
        }
    }
    systems == BTreeSet::from(["local".to_string(), "measurement".to_string()])
}
