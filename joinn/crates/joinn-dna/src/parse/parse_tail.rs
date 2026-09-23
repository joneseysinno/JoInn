//! `parse_tail`.

#![allow(clippy::result_large_err)]

use crate::model::{Allele, RegulatoryRegion};
use joinn_frame::{FrameRegistry, Verdict};

use crate::parse::Parser;

pub(in crate::parse) fn parse_tail(
    src: &str,
    frames: &FrameRegistry,
) -> Verdict<(RegulatoryRegion, Vec<Allele>)> {
    let mut p = Parser::new(src, frames);
    p.skip();
    let mut regulatory = RegulatoryRegion::default();
    let mut alleles = Vec::new();
    while !p.eof() {
        match p.peek_ident() {
            Some("regulatory") => {
                p.ident();
                if let Err(r) = p.expect('{') {
                    return Verdict::Refused(r);
                }
                regulatory = match p.regulatory_body() {
                    Verdict::Ok(r) => r,
                    Verdict::Refused(e) => return Verdict::Refused(e),
                };
                if let Err(r) = p.expect('}') {
                    return Verdict::Refused(r);
                }
            }
            Some("alleles") => {
                p.ident();
                if let Err(r) = p.expect('{') {
                    return Verdict::Refused(r);
                }
                alleles = match p.alleles_body() {
                    Verdict::Ok(a) => a,
                    Verdict::Refused(e) => return Verdict::Refused(e),
                };
                if let Err(r) = p.expect('}') {
                    return Verdict::Refused(r);
                }
            }
            Some(_) | None => p.advance(),
        }
        p.skip();
    }
    Verdict::Ok((regulatory, alleles))
}
