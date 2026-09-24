//! Gate 5 item 8 control: the artifact's text does not appear on the far side.

use super::subject::Subject;
use joinn_link::{format_link_refusal, Address, LinkRefusal, LinkRefusalKind};

pub(crate) fn g5_locality_control(subject: &Subject) -> bool {
    let Subject::Text(secret) = subject else {
        return true;
    };
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
