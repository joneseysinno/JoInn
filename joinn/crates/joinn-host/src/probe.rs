//! Read-only inspection of one port. Holds no grants, takes no budget, cannot fire.

use joinn_frame::{CheckId, Refusal, Verdict};
use joinn_live::BodyState;

use crate::description::{Description, describe};
use crate::intent::Address;

/// Read one port of an instance. The address's port selects the face.
pub fn probe(state: &BodyState, addr: Address) -> Verdict<Description> {
    let mut description = match describe(state, &addr.instance) {
        Verdict::Ok(d) => d,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    if let Some(reason) = state.last_refusal() {
        description.label = reason.to_owned();
    }
    let face = description
        .ports
        .iter()
        .find(|p| p.position == addr.port)
        .cloned();
    let Some(face) = face else {
        return Verdict::Refused(Refusal::structural(
            CheckId::Other,
            format!(
                "probe: {} has no port {}; acceptance is a declared port of that instance",
                addr.printed(),
                addr.port
            ),
        ));
    };
    description.ports = vec![face];
    Verdict::Ok(description)
}
