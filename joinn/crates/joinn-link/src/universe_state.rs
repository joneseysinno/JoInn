//! Universe runtime. Two phases, canonical order, one shared step budget.

mod inject;
mod new;
mod run;

use joinn_live::BodyState;
use std::collections::BTreeMap;

use crate::Address;
use crate::capability::LinkRuntime;
use crate::universe::Universe;

/// What crossed, or why a delivery stopped. No reason string.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinkRefusal {
    /// Link id.
    pub link: String,
    /// Body that was refused, or the head that did not receive.
    pub body: String,
    /// Member address on that body.
    pub member: Address,
    /// Closed kind. There is no text field.
    pub kind: LinkRefusalKind,
}

/// Why a delivery stopped. The refusing body's own words stay in that body.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LinkRefusalKind {
    /// The head body refused the value.
    Refused,
    /// An ordered link's capability is not held.
    CapabilityNotHeld,
    /// A second value for a port that already took one this pass.
    NotDelivered,
}

/// A body fired, or a link stopped.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum UniverseReport {
    /// `instance` fired inside `body`.
    Fired {
        /// Body alias.
        body: String,
        /// Instance that fired.
        instance: String,
    },
    /// A delivery did not land.
    Link(LinkRefusal),
}

/// Live bodies, one shared budget, deliveries in canonical order.
pub struct UniverseState {
    universe: Universe,
    bodies: BTreeMap<String, BodyState>,
    runtime: LinkRuntime,
    budget: u64,
    spent: u64,
}

impl UniverseState {
    /// Read a body's live state. Descriptions and probes use this.
    pub fn body(&self, alias: &str) -> Option<&BodyState> {
        self.bodies.get(alias)
    }

    /// The capability register. Grant and revoke go through this.
    pub fn link_runtime(&mut self) -> &mut LinkRuntime {
        &mut self.runtime
    }
}

/// Far-side line. The kind is a name, never the body's reason.
pub fn format_link_refusal(refusal: &LinkRefusal) -> String {
    format!(
        "link {} {} {}.{} {:?}",
        refusal.link, refusal.body, refusal.member.instance, refusal.member.port, refusal.kind
    )
}
