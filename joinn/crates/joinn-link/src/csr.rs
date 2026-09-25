//! Compressed sparse row incidence. Derived from a universe; never hashed.

mod from_universe;
mod to_coding;

pub use from_universe::csr_from_universe;
pub use to_coding::coding_from_csr;

use std::collections::BTreeMap;

use crate::universe::{Mark, Order};

/// One CSR member. Indices refer to the parallel name tables.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CsrMember {
    /// Index into `body_aliases`.
    pub body: u16,
    /// Index into `instances`.
    pub instance: u16,
    /// Port position.
    pub port: u32,
    /// Tail, head, or neither.
    pub mark: Mark,
}

/// Incidence of every link, derived from a parsed universe.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Csr {
    /// Codex copied from the universe.
    pub codex: u16,
    /// Body aliases in the order bodies were indexed.
    pub body_aliases: Vec<String>,
    /// Body hashes parallel to `body_aliases`.
    pub body_hashes: Vec<joinn_frame::Hash>,
    /// Instance names referenced by members.
    pub instances: Vec<String>,
    /// Link ids parallel to ranges in `link_offsets`.
    pub link_ids: Vec<String>,
    /// Link order flags parallel to `link_ids`.
    pub link_orders: Vec<Order>,
    /// Start index of each link's members in `members`, plus a terminator.
    pub link_offsets: Vec<u32>,
    /// Flat member table.
    pub members: Vec<CsrMember>,
    /// Declared capability grants.
    pub grants: BTreeMap<String, String>,
    /// Lenses restored when printing back.
    pub lenses: Vec<crate::universe::Lens>,
}
