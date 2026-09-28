//! Gate 4 helper: parse one corpus artifact by kind.

use std::fs;

use super::parse_subject::parse_subject;
use super::subject::Subject;
use super::workspace_root::workspace_root;

/// The parsed subject at `corpus/<rel>`, or `None` when it cannot be read or parsed.
pub(crate) fn g4_subject(rel: &str) -> Option<Subject> {
    let root = workspace_root().ok()?;
    let text = fs::read_to_string(root.join("corpus").join(rel)).ok()?;
    parse_subject(rel, &text).ok()
}
