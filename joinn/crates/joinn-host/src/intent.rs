//! Intents are derived from the body, never invented by a host.

mod check_intent;
mod intent_set;

pub use check_intent::check_intent;
pub use intent_set::intent_set;

pub use joinn_link::Address;

use joinn_frame::Term;

/// A host-emitted intent: an address in the body's set, and a raw term.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Intent {
    /// Where the term is aimed.
    pub address: Address,
    /// Raw term. The membrane checks it; the host does not.
    pub term: Term,
}
