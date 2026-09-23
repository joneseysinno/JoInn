//! The floor: irreducible and opposed. Admitted by declaration, not by the gate.
//!
//! Membership is the rule in docs/Findings/the-floor.md. The count is derived.

mod bind;
mod bound;
mod build_prim;
mod canon_int;
mod case_prim;
mod check_one;
mod check_oppositions;
mod check_pairing;
mod choose;
mod contains;
mod content_store;
mod eq_prim;
mod evaluable;
mod fan;
mod fill;
mod frame_for_build;
mod frame_of;
mod grant;
mod hash_prim;
mod int_tag;
mod join;
mod lying_case;
mod pair;
mod prim_cell;
mod prim_ports;
mod refuse;
mod register;
mod register_prims;
mod resolve_prim;
mod revoke;
mod split;
mod two;
mod unbind;

pub use build_prim::BuildPrim;
pub use case_prim::CasePrim;
pub use check_oppositions::check_oppositions;
pub use check_pairing::check_pairing;
pub use choose::Choose;
pub use contains::contains;
pub use eq_prim::EqPrim;
pub use hash_prim::HashPrim;
pub use lying_case::LyingCase;
pub use pair::Pair;
pub use prim_cell::prim_cell;
pub use prim_ports::prim_ports;
pub use register::register;
pub use register_prims::register_prims;
pub use resolve_prim::ResolvePrim;
pub use split::Split;

/// Stated inverse. Every member names an opposite that names it back.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Opposition {
    /// Named inverse in the floor.
    Inverse(&'static str),
    /// Neither an inverse. The pairing check refuses this.
    Undeclared,
}

/// Kind of floor pair. Enforced by `check::v33`: alleles name matter only.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Register {
    /// Evaluable, pure, frame-parametric. Alleles may name these.
    Matter,
    /// Body construction — grammar, not functions.
    Space,
    /// Engine services. Never an allele.
    Physics,
}

/// A floor primitive: an oracle that can state its opposition.
pub trait Reference: joinn_gate::Oracle {
    /// Stable name.
    fn name(&self) -> &'static str;
    /// The stated inverse.
    fn opposition(&self) -> Opposition;
    /// Matter, space, or physics.
    fn register(&self) -> Register;
}
