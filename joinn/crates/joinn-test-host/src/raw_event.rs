//! A scripted event: an address and a raw term.

use joinn_frame::Term;
use joinn_host::Address;

/// A scripted event: an address and a raw term.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RawEvent {
    /// Membrane address.
    pub address: Address,
    /// Raw term. The membrane checks it.
    pub term: Term,
}
