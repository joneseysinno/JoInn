//! Result of `Frame::case`.

use super::OpName;
use crate::value::Value;

/// Result of `Frame::case`. Constructors without a destructor are opposition missing.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Case {
    /// A declared generator: `zero`, `empty`, …
    Generator,
    /// A constructed value, with the op that built it and its parts.
    Built {
        /// Signature operation.
        op: OpName,
        /// Constructor arguments, each a value (possibly in another frame).
        parts: Vec<Value>,
    },
}
