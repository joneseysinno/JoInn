//! DNA structs. Regulatory data cannot implement `Genotype`.

mod cli_input_cell;
mod format_cell;
mod generate_int;
mod generate_text;
mod sum_cell;

pub use cli_input_cell::cli_input_cell;
pub use format_cell::format_cell;
pub use sum_cell::sum_cell;

use crate::formula::{Formula, Law, LawName};
use joinn_frame::{FrameRef, Hash, Value};
use std::collections::BTreeMap;

/// Port direction.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Direction {
    /// In-port.
    In,
    /// Out-port.
    Out,
}

/// Join policy. Hashed, unread in Phase 1. Default refuse.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum JoinPolicy {
    /// Silently dropping a message is the untrue option.
    Refuse,
    /// Latest-wins. Phase 2.
    Latest,
    /// Queue both. Phase 2.
    Queue,
}

/// A port declaration. Identity is (direction, frame, position).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PortDecl {
    /// Ordinal fixed at birth, never reused.
    pub position: u32,
    /// In or out.
    pub direction: Direction,
    /// Frame of values at this port.
    pub frame: FrameRef,
    /// Whether a message is required to fire.
    pub required: bool,
}

/// Reserved declaration. Must be empty in Phase 1.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Declaration {
    /// Source text. Any non-empty list is a gate refusal.
    pub text: String,
}

/// Contract: ports, tombstones, join policy, require/ensure.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Contract {
    /// Ordered by position.
    pub ports: Vec<PortDecl>,
    /// Tombstones. Hashed.
    pub retired: Vec<u32>,
    /// Default refuse.
    pub join_policy: JoinPolicy,
    /// Require predicates by port position.
    pub require: BTreeMap<u32, Formula>,
    /// Ensure predicates by port position.
    pub ensure: BTreeMap<u32, Formula>,
}

/// A founding or allele witness: in-map → out-map, by position.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Witness {
    /// Inputs by port position.
    pub inputs: BTreeMap<u32, Value>,
    /// Expected outputs by port position.
    pub outputs: BTreeMap<u32, Value>,
}

/// The hashed half of a cell.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CodingRegion {
    /// Canonical-form version. Phase 1 is 1.
    pub codex: u16,
    /// The frame truth is judged in.
    pub frame: FrameRef,
    /// Ports and join policy.
    pub contract: Contract,
    /// Named laws.
    pub laws: BTreeMap<LawName, Law>,
    /// Founding witnesses. Hashed.
    pub founding: Vec<Witness>,
    /// Must be empty in Phase 1.
    pub declarations: Vec<Declaration>,
    /// Parent coding-region hash, if any.
    pub lineage: Option<Hash>,
    /// Declared turns. Absent in print when empty (§2.6).
    pub turns: Vec<TurnDecl>,
}

/// `turn <out> from {positions}`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TurnDecl {
    /// Position computed by the turn.
    pub out: u32,
    /// Positions the turn allele reads.
    pub from: Vec<u32>,
}

/// Display names, prompts, styles. Never hashed.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct RegulatoryRegion {
    /// Port position → display name.
    pub names: BTreeMap<u32, String>,
    /// Named literals (prompt, …).
    pub literals: BTreeMap<String, String>,
    /// Named styles.
    pub styles: BTreeMap<String, String>,
    /// Port position → accessibility label. Regulatory.
    pub labels: BTreeMap<u32, String>,
}

/// Registered native name. The Rust body is not hashed.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct NativeId(pub String);

/// Phase 1 allele body. Phase 2 adds `Dna`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AlleleBody {
    /// A registered native allele.
    Native(NativeId),
    /// A reference allele: a body written over the floor.
    Dna(Hash),
}

/// Payload. Hashed on its own, never into the cell hash.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Allele {
    /// Frame this allele runs in.
    pub frame: FrameRef,
    /// Body.
    pub body: AlleleBody,
    /// This allele's own witness corpus.
    pub witnesses: Vec<Witness>,
}

/// A cell: coding + regulatory + alleles + lineage (lineage also lives on coding).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cell {
    /// Hashed.
    pub coding: CodingRegion,
    /// Never hashed.
    pub regulatory: RegulatoryRegion,
    /// Payload.
    pub alleles: Vec<Allele>,
}
