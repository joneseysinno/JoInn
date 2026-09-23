//! `parse_body`.

#![allow(clippy::result_large_err)]

use joinn_frame::{FrameRegistry, Verdict, nfc};

use crate::body::Body;
use crate::body::parse_body_coding::parse_body_coding;
use crate::body::parse_body_regulatory::parse_body_regulatory;
use crate::body::split_delimiter::split_delimiter;
use crate::body::strip_comments::strip_comments;

/// Parse a `.body` file.
pub fn parse_body(src: &str, _frames: &FrameRegistry) -> Verdict<Body> {
    let src = nfc(&src.replace("\r\n", "\n"));
    let stripped = strip_comments(&src);
    let (body_src, rest) = split_delimiter(&stripped);
    let coding = match parse_body_coding(body_src) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let regulatory = parse_body_regulatory(rest);
    Verdict::Ok(Body { coding, regulatory })
}
