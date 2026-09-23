//! The host protocol. Two hosts prove it.

mod marker;

use joinn_frame::Verdict;

use crate::description::Description;
use crate::intent::Intent;
use crate::signals::Signals;

/// A host shows descriptions, turns raw events into intents, and declares signals.
pub trait Host {
    /// Host-specific raw input. Opaque to the protocol.
    type Raw;

    /// Outward: show a description. The host decides how; the description decides what.
    fn present(&mut self, d: &Description) -> Verdict<()>;

    /// Inward: turn a host-specific raw event into a declared intent, or refuse it.
    fn intend(&mut self, raw: Self::Raw) -> Verdict<Option<Intent>>;

    /// Which environment signals this host emits.
    fn signals(&self) -> &Signals;
}
