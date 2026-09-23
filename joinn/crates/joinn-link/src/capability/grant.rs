//! Grant a capability across an ordered hyperedge.

use joinn_frame::{CheckId, Refusal, Verdict};

use super::LinkRuntime;
use crate::universe::{Order, Universe};

/// Grant delivery on `link_id` to `to`. The capability's name is the link id.
pub fn grant(
    runtime: &mut LinkRuntime,
    universe: &Universe,
    link_id: &str,
    to: &str,
) -> Verdict<()> {
    let Some(link) = universe.coding.links.iter().find(|l| l.id == link_id) else {
        return Verdict::Refused(Refusal::structural(
            CheckId::Grant,
            format!("link {link_id} is not in this universe; acceptance is a declared link"),
        ));
    };
    if link.order != Order::Ordered {
        return Verdict::Refused(Refusal::structural(
            CheckId::Grant,
            format!(
                "link {link_id} is not ordered; acceptance is an ordered hyperedge for a capability"
            ),
        ));
    }
    if !link.members.iter().any(|m| m.body == to) {
        return Verdict::Refused(Refusal::structural(
            CheckId::Grant,
            format!(
                "body {to} is not a member of link {link_id}; acceptance is a member of that link"
            ),
        ));
    }
    runtime.held.insert((link_id.to_owned(), to.to_owned()));
    Verdict::Ok(())
}
