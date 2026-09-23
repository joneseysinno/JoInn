//! A refusal is a value. `Result` is reserved for host failures.

mod structural;

use crate::hash::Hash;
use crate::value::Value;
use std::collections::BTreeMap;

/// Gate or frame check that produced a refusal.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CheckId {
    /// Contract well-formedness.
    Contract,
    /// Law sampling.
    Laws,
    /// Founding or testimony replay.
    Witnesses,
    /// Conservative extension / cross-frame.
    Extension,
    /// Frame obligation harness.
    FrameObligation,
    /// Source parse.
    Parse,
    /// Canonicalization of a term.
    Canonicalize,
    /// Degeneracy advisory (never a refusal of admission, but named).
    Advisory,
    /// Step budget exceeded. Distinct from every membrane refusal (V39).
    Budget,
    /// Join policy refused a second message.
    Join,
    /// Capability grant missing or stalled.
    Grant,
    /// V33: a reference genome that reaches only the floor, its cell, and seals.
    V33,
    /// Anything else, named in `reason`.
    Other,
}

/// What a refusal is about.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Subject {
    /// A frame, by its printed reference.
    Frame(String),
    /// A cell, by coding-region hash.
    Cell(Hash),
    /// An allele, by registered native name.
    Allele(String),
    /// A named law.
    Law(String),
    /// A founding witness.
    Witness,
    /// A value being parsed or canonicalized.
    Value,
    /// Catch-all.
    Other(String),
}

/// A shrunk counter-example. A refusal that needs one and lacks one is a bug.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CounterExample {
    /// Variable or port bindings that broke the check.
    pub bindings: BTreeMap<String, Value>,
    /// What the law or witness expected, if that is a single value.
    pub expected: Option<Value>,
    /// What the allele produced, if it produced anything.
    pub got: Option<Value>,
}

/// A refused verdict. Never converted into `Err`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Refusal {
    /// Which check refused.
    pub check: CheckId,
    /// What was being judged.
    pub subject: Subject,
    /// Human-readable reason. Not a substitute for `counterexample`.
    pub reason: String,
    /// Shrunk evidence, when the check is not purely structural.
    pub counterexample: Option<CounterExample>,
    /// Seed that produced the sample. Replays exactly.
    pub seed: u64,
}

/// JoInn's own result type. `Ok` is not `Result::Ok`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Verdict<T> {
    /// The check or operation succeeded.
    Ok(T),
    /// The check or operation refused, with evidence.
    Refused(Refusal),
}

impl<T> Verdict<T> {
    /// Successful verdict.
    pub fn accepted(value: T) -> Self {
        Verdict::Ok(value)
    }

    /// Refused verdict.
    pub fn refused(refusal: Refusal) -> Self {
        Verdict::Refused(refusal)
    }

    /// Map the accepted value.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Verdict<U> {
        match self {
            Verdict::Ok(v) => Verdict::Ok(f(v)),
            Verdict::Refused(r) => Verdict::Refused(r),
        }
    }

    /// Chain another verdict-producing step.
    pub fn and_then<U, F: FnOnce(T) -> Verdict<U>>(self, f: F) -> Verdict<U> {
        match self {
            Verdict::Ok(v) => f(v),
            Verdict::Refused(r) => Verdict::Refused(r),
        }
    }

    /// True when accepted.
    pub fn is_ok(&self) -> bool {
        matches!(self, Verdict::Ok(_))
    }

    /// True when refused.
    pub fn is_refused(&self) -> bool {
        matches!(self, Verdict::Refused(_))
    }

    /// Borrow the accepted value.
    pub fn as_ref(&self) -> Option<&T> {
        match self {
            Verdict::Ok(v) => Some(v),
            Verdict::Refused(_) => None,
        }
    }
}
