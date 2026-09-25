//! Revoke a capability that rode an ordered hyperedge.

use joinn_frame::{CheckId, Refusal, Verdict};

use super::LinkRuntime;

/// Remove `capability` from `from`. A missing grant is still Ok: revoke is idempotent.
pub fn revoke(runtime: &mut LinkRuntime, capability: &str, from: &str) -> Verdict<()> {
    let key = (capability.to_owned(), from.to_owned());
    if !runtime.held.remove(&key) {
        return Verdict::Refused(Refusal::structural(
            CheckId::Grant,
            format!("capability {capability} is not held by {from}; acceptance is a prior grant"),
        ));
    }
    Verdict::Ok(())
}
