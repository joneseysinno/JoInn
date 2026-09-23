//! Ask whether a body still holds a capability.

use joinn_frame::{CheckId, Refusal, Verdict};

use super::LinkRuntime;

/// Refuse naming the capability when it is not held.
pub fn check_capability(runtime: &LinkRuntime, capability: &str, body: &str) -> Verdict<()> {
    if runtime
        .held
        .contains(&(capability.to_owned(), body.to_owned()))
    {
        Verdict::Ok(())
    } else {
        Verdict::Refused(Refusal::structural(
            CheckId::Grant,
            format!(
                "capability {capability} is not held by {body}; acceptance is a grant over an ordered hyperedge"
            ),
        ))
    }
}
