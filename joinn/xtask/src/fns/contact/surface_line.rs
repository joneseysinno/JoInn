//! Two derivations of a contact body's surface, and of its intent set.

use joinn_dna::{Body, Cell, Contact, Direction};
use joinn_frame::{Hash, Verdict};
use joinn_link::{contact_surface, surface};
use std::collections::{BTreeMap, BTreeSet};

/// `surface equal (<n> ports)` when `contact_surface` equals the surface of the
/// lowered body and the two in-port address sets agree.
pub(super) fn surface_line(
    contact: &Contact,
    lowered: &Body,
    cells: &BTreeMap<Hash, Cell>,
) -> Result<String, String> {
    let own = match contact_surface(contact, cells) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(format!("surface: contact refused: {}", r.reason)),
    };
    let derived = match surface(lowered, cells) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(format!("surface: lowered refused: {}", r.reason)),
    };
    if own != derived {
        return Err(format!(
            "surface differs: contact {} port(s), lowered {} port(s)",
            own.len(),
            derived.len()
        ));
    }
    let own_intents: BTreeSet<String> = own
        .iter()
        .filter(|p| p.direction == Direction::In)
        .map(|p| p.address.printed())
        .collect();
    let lowered_intents: BTreeSet<String> = joinn_host::intent_set(lowered, cells)
        .iter()
        .map(|a| a.printed())
        .collect();
    if own_intents != lowered_intents {
        return Err(format!(
            "intent sets differ: contact {own_intents:?}, lowered {lowered_intents:?}"
        ));
    }
    Ok(format!("surface equal ({} ports)", own.len()))
}
