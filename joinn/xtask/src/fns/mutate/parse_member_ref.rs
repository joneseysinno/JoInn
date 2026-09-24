//! Parse `body.instance@port` from a catalogue member string.

use joinn_link::{Mark, Member};

use super::refuse::refuse;
use joinn_frame::Verdict;

pub(super) fn parse_member_ref(s: &str) -> Verdict<Member> {
    let Some((body, rest)) = s.split_once('.') else {
        return Verdict::Refused(refuse(format!(
            "member {s} is not body.instance@port; acceptance is that form"
        )));
    };
    let Some((instance, port_s)) = rest.split_once('@') else {
        return Verdict::Refused(refuse(format!(
            "member {s} is not body.instance@port; acceptance is that form"
        )));
    };
    let Ok(port) = port_s.parse::<u32>() else {
        return Verdict::Refused(refuse(format!(
            "member {s} port is not a number; acceptance is a port index"
        )));
    };
    Verdict::Ok(Member {
        body: body.to_owned(),
        instance: instance.to_owned(),
        port,
        mark: Mark::None,
    })
}
