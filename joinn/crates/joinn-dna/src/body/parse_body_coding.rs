//! `parse_body_coding`.

#![allow(clippy::result_large_err)]

use joinn_frame::Verdict;

use crate::body::{BodyCoding, BodyParser};

pub(in crate::body) fn parse_body_coding(src: &str) -> Verdict<BodyCoding> {
    let mut p = BodyParser::new(src);
    p.skip();
    if p.peek_ident() == Some("body") {
        p.ident();
        if let Err(r) = p.expect('{') {
            return Verdict::Refused(r);
        }
        let c = match p.coding_body() {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        if let Err(r) = p.expect('}') {
            return Verdict::Refused(r);
        }
        return Verdict::Ok(c);
    }
    p.coding_body()
}
