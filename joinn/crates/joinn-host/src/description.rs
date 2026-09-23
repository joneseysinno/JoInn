//! A cell's self-description. A value, not text.

mod describe;
mod describe_refusal;
mod hash_description;
mod print_description;
mod quoted;

pub use describe::describe;
pub use describe_refusal::describe_refusal;
pub use hash_description::hash_description;
pub use print_description::print_description;

use crate::role::Role;
use joinn_dna::Direction;
use joinn_frame::{FrameRef, Hash};

/// A cell describing itself. No formatting, no layout, no host knowledge.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Description {
    /// Coding-region hash of the cell.
    pub cell: Hash,
    /// Instance name in the body.
    pub instance: String,
    /// Ports in position order.
    pub ports: Vec<PortFace>,
    /// Regulatory face of the instance itself.
    pub label: String,
    /// Accessibility role of the instance.
    pub role: Role,
}

/// One port as the cell names it.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PortFace {
    /// Ordinal fixed at birth.
    pub position: u32,
    /// In or out.
    pub direction: Direction,
    /// Frame of values at this port.
    pub frame: FrameRef,
    /// Canonical printed form of the value, or None when the slot is empty.
    pub value: Option<String>,
    /// Regulatory display name (G1: names are regulatory, positions are structure).
    pub name: String,
    /// Accessibility label. Regulatory.
    pub label: String,
    /// Accessibility role of the port.
    pub role: Role,
}
