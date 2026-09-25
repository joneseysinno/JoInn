//! Refuse an intent whose address is not in the body's derived set.

use joinn_dna::{Body, Cell};
use joinn_frame::{CheckId, Hash, Refusal, Verdict};
use std::collections::BTreeMap;

use super::{Intent, intent_set};

/// A host may only emit an intent in `intent_set(body, cells)`.
pub fn check_intent(body: &Body, cells: &BTreeMap<Hash, Cell>, intent: &Intent) -> Verdict<()> {
    if intent_set(body, cells).contains(&intent.address) {
        Verdict::Ok(())
    } else {
        Verdict::Refused(Refusal::structural(
            CheckId::Other,
            format!(
                "intent {} is not in the body's intent set",
                intent.address.printed()
            ),
        ))
    }
}
