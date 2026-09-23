//! Law and formula AST. Primitive names are unrepresentable as frame ops.

use joinn_frame::{FrameRef, Hash, OpName, Value};
use std::collections::BTreeMap;

/// A bound variable.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct VarId(pub String);

/// Law name. Part of the coding region.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct LawName(pub String);

/// Term language of §4.4. Trailing underscore avoids colliding with `Term`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Term_ {
    /// Bound variable.
    Var(VarId),
    /// Frame-tagged literal.
    Lit(Value),
    /// An operation from a named frame's signature.
    FrameOp {
        /// Frame that owns the op.
        frame: FrameRef,
        /// Signature name.
        op: OpName,
        /// Arguments.
        args: Vec<Term_>,
    },
    /// The cell under definition, read at an out-port.
    SelfAt {
        /// Out-port position.
        out: u32,
        /// In-port position → term.
        args: BTreeMap<u32, Term_>,
    },
    /// Another coding region, by hash, read at an out-port.
    CellAt {
        /// Full content hash of the other coding region.
        cell: Hash,
        /// Out-port position.
        out: u32,
        /// In-port position → term.
        args: BTreeMap<u32, Term_>,
    },
}

/// Formula language. No existentials, no `let`, no recursion.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Formula {
    /// Equality in the term's frame.
    Eq(Term_, Term_),
    /// Negation.
    Not(Box<Formula>),
    /// Conjunction.
    And(Vec<Formula>),
    /// Disjunction.
    Or(Vec<Formula>),
    /// Implication.
    Implies(Box<Formula>, Box<Formula>),
    /// Universal quantification.
    ForAll {
        /// Binders, stored in name order when printed.
        vars: Vec<(VarId, FrameRef)>,
        /// Body.
        body: Box<Formula>,
    },
}

/// A named law.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Law {
    /// Name, hashed.
    pub name: LawName,
    /// The formula.
    pub formula: Formula,
}
