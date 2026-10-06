//! System form: growing bodies, and the forces from outside that reach them.
//! Data only.

use joinn_frame::{FrameRef, Hash};
use std::collections::BTreeMap;

use crate::body::contact::ForceKind;

/// One bound body: a contact by hash, under an alias.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct SystemBody {
    /// The contact's coding hash.
    pub contact: Hash,
    /// The name the system's forces use for it.
    pub alias: String,
}

/// A force on a body. It names no instance: it reaches every grown instance
/// of the body through its receptor.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SystemForce {
    /// Which force.
    pub kind: ForceKind,
    /// The frame it acts on.
    pub frame: FrameRef,
    /// The response law, pinned by coding hash.
    pub response: Hash,
    /// The response's name.
    pub name: String,
    /// The body alias it reaches.
    pub on: String,
}

/// Hashed half of a system.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SystemCoding {
    /// Canonical-form version. Phase 7.4 is 1.
    pub codex: u16,
    /// Bound bodies.
    pub bodies: Vec<SystemBody>,
    /// Forces.
    pub forces: Vec<SystemForce>,
    /// Parent system hash, if any.
    pub lineage: Option<Hash>,
}

/// Never hashed: names, present templates, and the starting shape.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct SystemRegulatory {
    /// Display names.
    pub names: BTreeMap<String, String>,
    /// Response → present template.
    pub present: BTreeMap<String, String>,
    /// Body alias → how many empty boxes the shell shows.
    pub waiting: BTreeMap<String, u32>,
}

/// A system: coding + regulatory.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct System {
    /// Hashed.
    pub coding: SystemCoding,
    /// Never hashed.
    pub regulatory: SystemRegulatory,
}
