//! `parse_cell`.

#![allow(clippy::result_large_err)]

use crate::model::Cell;
use joinn_frame::{FrameRegistry, Verdict, nfc};

use crate::parse::parse_coding::parse_coding;
use crate::parse::parse_tail::parse_tail;
use crate::parse::split_delimiter::split_delimiter;
use crate::parse::strip_comments::strip_comments;

/// Parse a full cell file (coding / delimiter / regulatory / alleles).
pub fn parse_cell(src: &str, frames: &FrameRegistry) -> Verdict<Cell> {
    let src = nfc(&src.replace("\r\n", "\n"));
    let stripped = strip_comments(&src);
    let (coding_src, rest) = split_delimiter(&stripped);
    let coding = match parse_coding(coding_src, frames) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let (regulatory, alleles) = match parse_tail(rest, frames) {
        Verdict::Ok(v) => v,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    Verdict::Ok(Cell {
        coding,
        regulatory,
        alleles,
    })
}
