//! Parse `instance@port` into a contact member.

use joinn_dna::Member;
use joinn_frame::Verdict;

use super::refuse::refuse;

pub(super) fn parse_contact_member(s: &str) -> Verdict<Member> {
    let parsed = s.split_once('@').and_then(|(instance, port)| {
        let port = port.parse::<u32>().ok()?;
        (!instance.is_empty()).then(|| Member {
            instance: instance.to_owned(),
            port,
        })
    });
    match parsed {
        Some(m) => Verdict::Ok(m),
        None => Verdict::Refused(refuse(format!(
            "{s} is not a member; acceptance is instance@port"
        ))),
    }
}
