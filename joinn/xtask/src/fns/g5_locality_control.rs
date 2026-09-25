//! Gate 5 item 8 control: true when the hosts' far-side output contains the text.

use super::subject::Subject;
use joinn_link::{Address, LinkRefusal, LinkRefusalKind, format_link_refusal};

pub(crate) fn g5_locality_control(subject: &Subject) -> bool {
    let Subject::Text(secret) = subject else {
        return true;
    };
    if secret.is_empty() {
        return true;
    }
    // Far-side of a link refusal names the link id and never the body's reason.
    let far = format_link_refusal(&LinkRefusal {
        link: "e0".into(),
        body: "units".into(),
        member: Address {
            instance: "scale".into(),
            port: 0,
        },
        kind: LinkRefusalKind::Refused,
    });
    far.contains(secret.as_str())
}
