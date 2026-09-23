//! Frame identity, signatures, and the `Frame` trait.

mod case;

pub use case::Case;

use crate::term::Term;
use crate::value::Value;
use crate::verdict::Verdict;
use std::collections::BTreeMap;
use std::fmt;

/// The three Phase 1 frames. Unknown names are a parse refusal.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FrameId {
    /// Unicode strings.
    Text,
    /// Arbitrary-precision integers (ℤ).
    Int,
    /// Exact rationals (ℚ).
    Rat,
}

impl FrameId {
    /// Canonical printed name.
    pub fn as_str(self) -> &'static str {
        match self {
            FrameId::Text => "Text",
            FrameId::Int => "ℤ",
            FrameId::Rat => "ℚ",
        }
    }

    /// Parse a frame name, including source aliases `Z` and `Q`.
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "Text" => Some(FrameId::Text),
            "ℤ" | "Z" => Some(FrameId::Int),
            "ℚ" | "Q" => Some(FrameId::Rat),
            _ => None,
        }
    }
}

impl fmt::Display for FrameId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A named frame at a version.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FrameRef {
    /// Which frame.
    pub id: FrameId,
    /// Frame version. Phase 1 is `1` for all three.
    pub version: u32,
}

impl FrameRef {
    /// Convenience constructor.
    pub fn new(id: FrameId, version: u32) -> Self {
        Self { id, version }
    }

    /// Phase 1 `Text`.
    pub fn text() -> Self {
        Self::new(FrameId::Text, 1)
    }

    /// Phase 1 `ℤ`.
    pub fn int() -> Self {
        Self::new(FrameId::Int, 1)
    }

    /// Phase 1 `ℚ`.
    pub fn rat() -> Self {
        Self::new(FrameId::Rat, 1)
    }
}

impl fmt::Display for FrameRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.id, self.version)
    }
}

/// An operation name from a frame signature.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct OpName(pub String);

impl OpName {
    /// Borrow the name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for OpName {
    fn from(s: &str) -> Self {
        OpName(s.to_owned())
    }
}

/// The operations a frame exposes. Kept small on purpose (§2.4).
#[derive(Clone, Debug, Default)]
pub struct Signature {
    ops: BTreeMap<OpName, usize>,
}

impl Signature {
    /// Empty signature.
    pub fn new() -> Self {
        Self {
            ops: BTreeMap::new(),
        }
    }

    /// Insert an op and its arity.
    pub fn insert(&mut self, name: impl Into<OpName>, arity: usize) {
        self.ops.insert(name.into(), arity);
    }

    /// Look up an arity.
    pub fn arity(&self, name: &OpName) -> Option<usize> {
        self.ops.get(name).copied()
    }

    /// Iterate in name order.
    pub fn iter(&self) -> impl Iterator<Item = (&OpName, usize)> {
        self.ops.iter().map(|(n, a)| (n, *a))
    }

    /// True when the name is in the signature.
    pub fn contains(&self, name: &OpName) -> bool {
        self.ops.contains_key(name)
    }
}

/// A declared context of truth. Object-safe.
pub trait Frame: Send + Sync {
    /// This frame's reference.
    fn reference(&self) -> FrameRef;
    /// Minimal signature.
    fn signature(&self) -> &Signature;
    /// Apply a signature operation. Never panics; out-of-frame args refuse.
    fn apply_op(&self, op: &OpName, args: &[Value]) -> Verdict<Value>;
    /// Whether the term is a (not necessarily canonical) inhabitant.
    fn contains(&self, t: &Term) -> bool;
    /// Produce the unique canonical value, or refuse.
    fn canonicalize(&self, t: Term) -> Verdict<Value>;
    /// Equality coincides with canonical identity (FO1).
    fn eq(&self, a: &Value, b: &Value) -> bool;
    /// Canonical literal text of the term (no frame prefix).
    fn print(&self, v: &Value) -> String;
    /// Parse a canonical or source literal of this frame.
    fn parse(&self, s: &str) -> Verdict<Value>;
    /// Deterministic generator.
    fn generate(&self, seed: u64, size: u8) -> Value;
    /// Strictly simpler candidates for shrinking. Must terminate, never grow.
    fn shrink(&self, v: &Value) -> Vec<Value>;
    /// Frames this one extends.
    fn extensions(&self) -> &[FrameRef];
    /// Embedding ι from a narrower frame.
    fn embed(&self, from: &FrameRef, v: &Value) -> Verdict<Value>;
    /// Restriction ρ to a narrower frame (partial).
    fn restrict(&self, to: &FrameRef, v: &Value) -> Verdict<Value>;
    /// Eliminator: which constructor made this value (§2.1).
    fn case(&self, v: &Value) -> Case;
    /// Ordered constructors. Index 0 is the generator; `k ≥ 1` is the k-th
    /// constructor. The order is part of the frame's identity: appending is
    /// conservative, reordering or removing bumps the frame version.
    fn constructors(&self) -> &[OpName];
}
