//! `sum@2` after §2.12's script, as the test host describes it.

use std::collections::BTreeMap;

use joinn_dna::{Cell, Contact};
use joinn_frame::{Hash, Verdict};
use joinn_host::Role;
use joinn_test_host::run_contact;

use super::contact::script_events;

/// The value on the last cell description of `sum` at position 2, or `None`
/// when the contact is refused or `sum` never fired.
pub(crate) fn g7_sum_after(contact: &Contact, cells: &BTreeMap<Hash, Cell>) -> Option<String> {
    let Verdict::Ok(cap) = run_contact(
        contact,
        cells.clone(),
        joinn_prim::sealed_natives(),
        script_events(),
    ) else {
        return None;
    };
    cap.descriptions
        .iter()
        .rev()
        .find(|d| d.role == Role::Cell && d.instance == "sum")
        .and_then(|d| d.ports.iter().find(|p| p.position == 2))
        .and_then(|p| p.value.clone())
}
