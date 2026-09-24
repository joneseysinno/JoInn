//! Parse `instance@port` from a catalogue wire endpoint.

use super::refuse::refuse;
use joinn_frame::Verdict;

pub(super) fn parse_endpoint(s: &str) -> Verdict<(String, u32)> {
    let Some((instance, port_s)) = s.split_once('@') else {
        return Verdict::Refused(refuse(format!(
            "endpoint {s} is not instance@port; acceptance is that form"
        )));
    };
    let Ok(port) = port_s.parse::<u32>() else {
        return Verdict::Refused(refuse(format!(
            "endpoint {s} port is not a number; acceptance is a port index"
        )));
    };
    Verdict::Ok((instance.to_owned(), port))
}
