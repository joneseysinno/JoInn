//! `parse_contact`.

use joinn_frame::{FrameRegistry, Verdict, nfc};

use crate::body::BodyParser;
use crate::body::contact::Contact;
use crate::body::parse_body_regulatory::parse_body_regulatory;
use crate::body::split_delimiter::split_delimiter;
use crate::body::strip_comments::strip_comments;

/// Parse a `.contact` file. The regulatory region is a body's, unchanged.
pub fn parse_contact(src: &str, _frames: &FrameRegistry) -> Verdict<Contact> {
    let src = nfc(&src.replace("\r\n", "\n"));
    let stripped = strip_comments(&src);
    let (coding_src, rest) = split_delimiter(&stripped);
    let mut p = BodyParser::new(coding_src);
    p.skip();
    let braced = p.peek_ident() == Some("contact");
    if braced {
        p.ident();
        if let Err(r) = p.expect('{') {
            return Verdict::Refused(r);
        }
    }
    let coding = match p.contact_coding() {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    if braced && let Err(r) = p.expect('}') {
        return Verdict::Refused(r);
    }
    p.skip();
    if !p.eof() {
        return Verdict::Refused(p.refuse(format!(
            "contact: text after the coding region: {:?}; acceptance is --- then the regulatory region",
            p.rest().lines().next().unwrap_or("")
        )));
    }
    let regulatory = parse_body_regulatory(rest);
    Verdict::Ok(Contact { coding, regulatory })
}
