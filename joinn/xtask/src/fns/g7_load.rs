//! The corpus contact calculator, its cells, and its lowered body.

use std::collections::BTreeMap;

use joinn_dna::{Body, Cell, Contact};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_link::lower;

use super::layout::corpus_contacts;

/// Every gate 7 item reads this file.
const REL: &str = "phase7/calculator.contact";

/// The contact, its lowered body, and the corpus cells. A refusal is `Err`:
/// P7-07 admits the corpus contact, so there is nothing to judge without it.
pub(crate) fn g7_load() -> Result<(Contact, Body, BTreeMap<Hash, Cell>), String> {
    let (contacts, cells) = corpus_contacts()?;
    let contact = match contacts.into_iter().find(|(r, _)| r == REL) {
        Some((_, Verdict::Ok(c))) => c,
        Some((_, Verdict::Refused(r))) => return Err(format!("{REL}: {}", r.reason)),
        None => return Err(format!("{REL} is not in the corpus")),
    };
    let lowered = match lower(&contact, &cells, &FrameRegistry::phase1()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Err(format!("{REL}: {}", r.reason)),
    };
    Ok((contact, lowered, cells))
}
