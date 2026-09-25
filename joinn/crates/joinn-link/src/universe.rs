//! A universe: bodies, links, and lenses. Hashed as joinn.universe.v1.

mod hash_universe;
mod parse_universe;
mod print_universe;

pub use hash_universe::hash_universe;
pub use parse_universe::parse_universe;
pub use print_universe::print_universe;

use joinn_frame::Hash;
use std::collections::BTreeMap;

/// Tail, head, or unmarked. Zero tails is a pure relation and is legal.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Mark {
    /// No mark.
    None,
    /// Tail of the link.
    Tail,
    /// Head of the link.
    Head,
}

/// Declared order. Never inferred.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Order {
    /// Members are a set; canonical print sorts them.
    None,
    /// Members are a sequence; canonical print keeps declaration order.
    Ordered,
}

/// One port on one body, named by that body's alias.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Member {
    /// Body alias declared in `bodies`.
    pub body: String,
    /// Instance name inside that body.
    pub instance: String,
    /// Port position.
    pub port: u32,
    /// Tail, head, or neither.
    pub mark: Mark,
}

/// A hyperedge. Communication between bodies.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Link {
    /// Link id.
    pub id: String,
    /// Declared order.
    pub order: Order,
    /// Members. Order matters when `order` is `Ordered`.
    pub members: Vec<Member>,
}

/// One body binding: hash and alias.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BodyBinding {
    /// Coding hash of the body.
    pub hash: Hash,
    /// Alias used by links and lenses.
    pub alias: String,
}

/// Bodies that share one system.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct System {
    /// System name.
    pub name: String,
    /// Body aliases, declaration order.
    pub bodies: Vec<String>,
}

/// Systems that share one galaxy.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Galaxy {
    /// Galaxy name.
    pub name: String,
    /// Systems.
    pub systems: Vec<System>,
}

/// One lens: a partitioning of bodies into systems.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Lens {
    /// Lens name.
    pub name: String,
    /// Galaxies.
    pub galaxies: Vec<Galaxy>,
}

/// Hashed half of a universe.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UniverseCoding {
    /// Codex. Phase 5 is 1.
    pub codex: u16,
    /// Body bindings.
    pub bodies: Vec<BodyBinding>,
    /// Links.
    pub links: Vec<Link>,
    /// Cross-body wires written in a universe. Always refused by Law 4.
    pub cross_wires: Vec<CrossWire>,
    /// Capability grants: ordered link id → receiving body alias.
    pub grants: BTreeMap<String, String>,
    /// Lenses.
    pub lenses: Vec<Lens>,
}

/// A wire written between bodies. The wrong container for a wire.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CrossWire {
    /// Source body alias.
    pub src_body: String,
    /// Source instance.
    pub src_instance: String,
    /// Source port.
    pub src_port: u32,
    /// Destination body alias.
    pub dst_body: String,
    /// Destination instance.
    pub dst_instance: String,
    /// Destination port.
    pub dst_port: u32,
}

/// Names and labels. Never hashed.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct UniverseRegulatory {
    /// Display names.
    pub names: BTreeMap<String, String>,
    /// Accessibility labels.
    pub labels: BTreeMap<String, String>,
}

/// A universe: coding + regulatory.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Universe {
    /// Hashed.
    pub coding: UniverseCoding,
    /// Never hashed.
    pub regulatory: UniverseRegulatory,
}
