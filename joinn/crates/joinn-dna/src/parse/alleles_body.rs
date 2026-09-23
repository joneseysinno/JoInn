//! `Parser::alleles_body`.

#![allow(clippy::result_large_err)]

use crate::model::Allele;
use joinn_frame::Verdict;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn alleles_body(&mut self) -> Verdict<Vec<Allele>> {
        let mut out = Vec::new();
        loop {
            self.skip();
            if self.peek() == Some('}') || self.eof() {
                break;
            }
            if self.peek_ident() == Some("allele") {
                self.ident();
                match self.allele_body() {
                    Verdict::Ok(a) => out.push(a),
                    Verdict::Refused(r) => return Verdict::Refused(r),
                }
            } else {
                self.advance();
            }
        }
        Verdict::Ok(out)
    }
}
