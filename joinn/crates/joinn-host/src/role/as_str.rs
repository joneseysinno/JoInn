//! Canonical printed token for a role.

use super::Role;

impl Role {
    /// Canonical printed token.
    pub fn as_str(self) -> &'static str {
        match self {
            Role::Cell => "cell",
            Role::Input => "input",
            Role::Output => "output",
            Role::Refusal => "refusal",
        }
    }
}
