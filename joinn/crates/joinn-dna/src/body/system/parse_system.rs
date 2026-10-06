//! `parse_system`.

use joinn_frame::{Verdict, nfc};

use crate::body::BodyParser;
use crate::body::split_delimiter::split_delimiter;
use crate::body::strip_comments::strip_comments;
use crate::body::system::System;

use super::parse_system_regulatory::parse_system_regulatory;

/// Parse a `.system` file: the coding region, then `---` and the regulatory
/// region.
pub fn parse_system(src: &str) -> Verdict<System> {
    let src = nfc(&src.replace("\r\n", "\n"));
    let stripped = strip_comments(&src);
    let (coding_src, rest) = split_delimiter(&stripped);
    let mut p = BodyParser::new(coding_src);
    p.skip();
    let braced = p.peek_ident() == Some("system");
    if braced {
        p.ident();
        if let Err(r) = p.expect('{') {
            return Verdict::Refused(r);
        }
    }
    let coding = match p.system_coding() {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    if braced && let Err(r) = p.expect('}') {
        return Verdict::Refused(r);
    }
    p.skip();
    if !p.eof() {
        return Verdict::Refused(p.refuse(format!(
            "system: text after the coding region: {:?}; acceptance is --- then the regulatory region",
            p.rest().lines().next().unwrap_or("")
        )));
    }
    Verdict::Ok(System {
        coding,
        regulatory: parse_system_regulatory(rest),
    })
}
