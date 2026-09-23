//! A far-side line has no place to put a body's reason.

use joinn_link::{Address, LinkRefusal, LinkRefusalKind, format_link_refusal};

#[test]
fn link_refusal_has_no_reason_text() {
    let secret = "secret membrane detail that must not cross";
    let far = format_link_refusal(&LinkRefusal {
        link: "e0".into(),
        body: "units".into(),
        member: Address {
            instance: "scale".into(),
            port: 0,
        },
        kind: LinkRefusalKind::Refused,
    });
    assert!(!far.contains(secret), "{far}");
    assert!(far.contains("e0"), "{far}");
    assert!(far.contains("Refused"), "{far}");
}
