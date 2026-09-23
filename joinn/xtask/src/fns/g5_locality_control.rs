//! Gate 5 item 8 control: the artifact's text does not appear on the far side.

use joinn_link::{format_link_refusal, Address, LinkRefusal, LinkRefusalKind};

pub(crate) fn g5_locality_control(art: &joinn_gate::Artifact) -> bool {
    let Ok(text) = std::str::from_utf8(art.bytes) else {
        return true;
    };
    let secret = text.trim();
    if secret.is_empty() {
        return true;
    }
    let far = format_link_refusal(&LinkRefusal {
        link: "e0".into(),
        body: "units".into(),
        member: Address {
            instance: "scale".into(),
            port: 0,
        },
        kind: LinkRefusalKind::Refused,
    });
    far.contains(secret)
}
