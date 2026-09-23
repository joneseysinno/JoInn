//! `parse_coding`.

#![allow(clippy::result_large_err)]

use crate::model::CodingRegion;
use joinn_frame::{FrameRegistry, Verdict, nfc};

use crate::parse::Parser;
use crate::parse::strip_comments::strip_comments;

/// Parse a coding region, with or without a wrapping `coding { … }`.
pub fn parse_coding(src: &str, frames: &FrameRegistry) -> Verdict<CodingRegion> {
    let src = nfc(&src.replace("\r\n", "\n"));
    let stripped = strip_comments(&src);
    let mut p = Parser::new(&stripped, frames);
    p.skip();
    if p.peek_ident() == Some("coding") {
        p.ident();
        if let Err(r) = p.expect('{') {
            return Verdict::Refused(r);
        }
        let region = match p.coding_body() {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        if let Err(r) = p.expect('}') {
            return Verdict::Refused(r);
        }
        return Verdict::Ok(region);
    }
    p.coding_body()
}
